use minerva_application::TaskTreeNode;
use minerva_domain::{Task, TaskId};
use std::collections::{BTreeMap, BTreeSet};

pub struct Graph {
    pub tasks: BTreeMap<TaskId, Task>,
    pub roots: Vec<TaskId>,
    parents: BTreeMap<TaskId, Option<TaskId>>,
    children: BTreeMap<Option<TaskId>, Vec<TaskId>>,
}

impl Graph {
    pub fn build(tree: &[TaskTreeNode]) -> Self {
        let mut graph = Self {
            tasks: BTreeMap::new(),
            roots: tree.iter().map(|node| node.task.id).collect(),
            parents: BTreeMap::new(),
            children: BTreeMap::new(),
        };
        flatten(tree, None, &mut graph);
        graph
    }

    pub fn parent(&self, task_id: TaskId) -> Option<TaskId> {
        self.parents.get(&task_id).copied().flatten()
    }

    pub fn visible_children(
        &self,
        task_id: TaskId,
        visible: &BTreeSet<TaskId>,
    ) -> Vec<TaskId> {
        self.children(task_id)
            .into_iter()
            .filter(|child| visible.contains(child))
            .collect()
    }

    pub fn children(&self, task_id: TaskId) -> Vec<TaskId> {
        self.children.get(&Some(task_id)).into_iter().flatten().copied().collect()
    }
}

fn flatten(tree: &[TaskTreeNode], parent_id: Option<TaskId>, graph: &mut Graph) {
    let mut stack = tree.iter().rev().map(|node| (node, parent_id)).collect::<Vec<_>>();
    while let Some((node, parent_id)) = stack.pop() {
        graph.tasks.insert(node.task.id, node.task.clone());
        graph.parents.insert(node.task.id, parent_id);
        graph.children.entry(parent_id).or_default().push(node.task.id);
        stack.extend(
            node.children.iter().rev().map(|child| (child, Some(node.task.id))),
        );
    }
}
