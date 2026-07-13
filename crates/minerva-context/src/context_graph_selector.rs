use crate::{
    ContextGraphSelection, ContextInclusionReason, ContextRelationshipDirection,
    ContextSelectionItem,
};
use minerva_domain::{
    ArchiveState, ContextPolicy, MinervaError, Relationship, Task, TaskId,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

type DependencyEdge = (TaskId, minerva_domain::RelationshipType);
type RelatedEdge =
    (TaskId, minerva_domain::RelationshipType, ContextRelationshipDirection);
type RelationshipIndexes =
    (BTreeMap<TaskId, Vec<DependencyEdge>>, BTreeMap<TaskId, Vec<RelatedEdge>>);

pub struct ContextGraphSelector<'a> {
    tasks: BTreeMap<TaskId, &'a Task>,
    children: BTreeMap<TaskId, Vec<TaskId>>,
    dependencies: BTreeMap<TaskId, Vec<DependencyEdge>>,
    related: BTreeMap<TaskId, Vec<RelatedEdge>>,
}

impl<'a> ContextGraphSelector<'a> {
    pub fn new(
        tasks: &'a [Task],
        relationships: &'a [Relationship],
    ) -> Result<Self, MinervaError> {
        let tasks =
            tasks.iter().map(|task| (task.id, task)).collect::<BTreeMap<_, _>>();
        let children = index_children(&tasks)?;
        let (dependencies, related) = index_relationships(&tasks, relationships)?;
        Ok(Self { tasks, children, dependencies, related })
    }

    pub fn select(
        &self,
        target: TaskId,
        policy: &ContextPolicy,
    ) -> Result<ContextGraphSelection, MinervaError> {
        let mut selection = ContextGraphSelection::default();
        let mut selected = BTreeSet::new();
        self.push(
            &mut selection,
            &mut selected,
            target,
            ContextInclusionReason::Target,
            policy,
        )?;
        self.ancestors(&mut selection, &mut selected, target, policy)?;
        self.dependencies(&mut selection, &mut selected, target, policy)?;
        self.related(&mut selection, &mut selected, target, policy)?;
        self.children(&mut selection, &mut selected, target, policy)?;
        self.siblings(&mut selection, &mut selected, target, policy)?;
        Ok(selection)
    }

    fn ancestors(
        &self,
        selection: &mut ContextGraphSelection,
        selected: &mut BTreeSet<TaskId>,
        target: TaskId,
        policy: &ContextPolicy,
    ) -> Result<(), MinervaError> {
        let Some(rule) = &policy.ancestors else {
            return Ok(());
        };
        let mut chain = Vec::new();
        let mut current = target;
        for depth in 1..=rule.depth {
            let Some(parent) = self.task(current)?.parent_id else { break };
            chain.push((parent, depth));
            current = parent;
        }
        for (task_id, depth) in chain.into_iter().rev() {
            self.push(
                selection,
                selected,
                task_id,
                ContextInclusionReason::Ancestor { depth },
                policy,
            )?;
        }
        Ok(())
    }

    fn dependencies(
        &self,
        selection: &mut ContextGraphSelection,
        selected: &mut BTreeSet<TaskId>,
        target: TaskId,
        policy: &ContextPolicy,
    ) -> Result<(), MinervaError> {
        let Some(rule) = &policy.dependencies else {
            return Ok(());
        };
        self.walk(
            target,
            rule.depth,
            |task_id, depth| {
                self.dependencies
                    .get(&task_id)
                    .into_iter()
                    .flatten()
                    .copied()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|(next, relationship_type)| {
                        (
                            next,
                            ContextInclusionReason::Dependency {
                                depth,
                                relationship_type,
                            },
                        )
                    })
                    .collect()
            },
            selection,
            selected,
            policy,
        )
    }

    fn related(
        &self,
        selection: &mut ContextGraphSelection,
        selected: &mut BTreeSet<TaskId>,
        target: TaskId,
        policy: &ContextPolicy,
    ) -> Result<(), MinervaError> {
        let Some(rule) = &policy.related_tasks else {
            return Ok(());
        };
        self.walk(
            target,
            rule.depth,
            |task_id, depth| {
                self.related
                    .get(&task_id)
                    .into_iter()
                    .flatten()
                    .copied()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|(next, relationship_type, direction)| {
                        (
                            next,
                            ContextInclusionReason::RelatedTask {
                                depth,
                                relationship_type,
                                direction,
                            },
                        )
                    })
                    .collect()
            },
            selection,
            selected,
            policy,
        )
    }

    fn children(
        &self,
        selection: &mut ContextGraphSelection,
        selected: &mut BTreeSet<TaskId>,
        target: TaskId,
        policy: &ContextPolicy,
    ) -> Result<(), MinervaError> {
        let Some(rule) = &policy.children else {
            return Ok(());
        };
        self.walk(
            target,
            rule.depth,
            |task_id, depth| {
                self.children
                    .get(&task_id)
                    .into_iter()
                    .flatten()
                    .copied()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|next| (next, ContextInclusionReason::Child { depth }))
                    .collect()
            },
            selection,
            selected,
            policy,
        )
    }

    fn siblings(
        &self,
        selection: &mut ContextGraphSelection,
        selected: &mut BTreeSet<TaskId>,
        target: TaskId,
        policy: &ContextPolicy,
    ) -> Result<(), MinervaError> {
        let Some(rule) = &policy.siblings else {
            return Ok(());
        };
        self.walk(
            target,
            rule.depth,
            |task_id, depth| {
                self.task(task_id)
                    .ok()
                    .and_then(|task| task.parent_id)
                    .and_then(|parent| self.children.get(&parent).cloned())
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|sibling| *sibling != task_id)
                    .map(|next| (next, ContextInclusionReason::Sibling { depth }))
                    .collect()
            },
            selection,
            selected,
            policy,
        )
    }

    fn walk(
        &self,
        target: TaskId,
        max_depth: u8,
        neighbors: impl Fn(TaskId, u8) -> Vec<(TaskId, ContextInclusionReason)>,
        selection: &mut ContextGraphSelection,
        selected: &mut BTreeSet<TaskId>,
        policy: &ContextPolicy,
    ) -> Result<(), MinervaError> {
        let mut queue = VecDeque::from([(target, 0_u8)]);
        let mut visited = BTreeSet::from([target]);
        while let Some((task_id, depth)) = queue.pop_front() {
            if depth == max_depth {
                continue;
            }
            for (next, reason) in neighbors(task_id, depth + 1) {
                if visited.insert(next) {
                    self.push(selection, selected, next, reason, policy)?;
                    queue.push_back((next, depth + 1));
                }
            }
        }
        Ok(())
    }

    fn push(
        &self,
        selection: &mut ContextGraphSelection,
        selected: &mut BTreeSet<TaskId>,
        task_id: TaskId,
        reason: ContextInclusionReason,
        policy: &ContextPolicy,
    ) -> Result<(), MinervaError> {
        let task = self.task(task_id)?;
        if reason != ContextInclusionReason::Target && !eligible(task, policy) {
            return Ok(());
        }
        if selected.insert(task_id) {
            selection.items.push(ContextSelectionItem { task: task.clone(), reason });
        }
        Ok(())
    }

    fn task(&self, task_id: TaskId) -> Result<&Task, MinervaError> {
        self.tasks
            .get(&task_id)
            .copied()
            .ok_or_else(|| MinervaError::TaskNotFound { task_ref: task_id.to_string() })
    }
}

fn index_children(
    tasks: &BTreeMap<TaskId, &Task>,
) -> Result<BTreeMap<TaskId, Vec<TaskId>>, MinervaError> {
    let mut children = BTreeMap::new();
    for task in tasks.values() {
        if let Some(parent) = task.parent_id {
            if !tasks.contains_key(&parent) {
                return Err(MinervaError::TaskNotFound {
                    task_ref: parent.to_string(),
                });
            }
            children.entry(parent).or_insert_with(Vec::new).push(task.id);
        }
    }
    Ok(children)
}

fn index_relationships(
    tasks: &BTreeMap<TaskId, &Task>,
    relationships: &[Relationship],
) -> Result<RelationshipIndexes, MinervaError> {
    let mut dependencies = BTreeMap::new();
    let mut related = BTreeMap::new();
    for relationship in relationships {
        validate_endpoint(tasks, relationship.source_task)?;
        validate_endpoint(tasks, relationship.target_task)?;
        if let Some((source, target)) = relationship
            .relationship_type
            .dependency_edge(relationship.source_task, relationship.target_task)
        {
            dependencies
                .entry(source)
                .or_insert_with(Vec::new)
                .push((target, relationship.relationship_type));
            continue;
        }
        if relationship.relationship_type == minerva_domain::RelationshipType::Parent {
            continue;
        }
        related.entry(relationship.source_task).or_insert_with(Vec::new).push((
            relationship.target_task,
            relationship.relationship_type,
            ContextRelationshipDirection::Outgoing,
        ));
        related.entry(relationship.target_task).or_insert_with(Vec::new).push((
            relationship.source_task,
            relationship.relationship_type,
            ContextRelationshipDirection::Incoming,
        ));
    }
    sort_edges(&mut dependencies);
    sort_edges(&mut related);
    Ok((dependencies, related))
}

fn validate_endpoint(
    tasks: &BTreeMap<TaskId, &Task>,
    task_id: TaskId,
) -> Result<(), MinervaError> {
    tasks
        .contains_key(&task_id)
        .then_some(())
        .ok_or_else(|| MinervaError::TaskNotFound { task_ref: task_id.to_string() })
}

fn sort_edges<T: Ord>(index: &mut BTreeMap<TaskId, Vec<T>>) {
    for values in index.values_mut() {
        values.sort_unstable();
    }
}

fn eligible(task: &Task, policy: &ContextPolicy) -> bool {
    (policy.include_archived || task.archive_state != ArchiveState::Archived)
        && (policy.include_completed
            || (task.completed_at.is_none() && task.status.as_str() != "completed"))
}
