use crate::context_compilation_freshness::ensure_fresh_declarations;
use crate::context_compilation_sections::build_sections;
use crate::{
    ContextCompilationError, ContextCompilationRequest, ContextCompilationResult,
    ContextDocument, ContextGraphSelector,
};
use minerva_application::{ProjectRepository, TaskRepository};
use std::path::Path;

pub struct ContextCompilationService;

impl ContextCompilationService {
    pub fn compile(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        start: &Path,
        request: &ContextCompilationRequest,
    ) -> Result<ContextCompilationResult, ContextCompilationError> {
        let root = project_repo.locate_project_root(start)?;
        let project = project_repo.load_project(&root)?;
        let policy = request.policy.clone().unwrap_or(project.context_policy);
        let target = task_repo.resolve_task(&root, &request.task_ref)?;
        let tasks = task_repo.list_tasks(&root)?;
        let relationships = task_repo.list_relationships(&root)?;
        let selection = ContextGraphSelector::new(&tasks, &relationships)?
            .select(target.id, &policy)?;
        ensure_fresh_declarations(task_repo, &root, &selection, &policy)?;
        let document = ContextDocument::new(build_sections(
            project_repo,
            task_repo,
            &root,
            &policy,
            &target,
            &selection,
        )?);
        if let Some(budget) = request.budget {
            let report = document.enforce_budget(budget)?;
            return Ok(ContextCompilationResult {
                markdown: report.render_with_manifest(),
                manifest: report.manifest(),
                estimated_tokens: report.document().total_estimated_tokens(),
                selection,
                excluded_sections: report.excluded_sections().to_vec(),
            });
        }
        Ok(ContextCompilationResult {
            markdown: document.render_with_manifest(),
            manifest: document.manifest(),
            estimated_tokens: document.total_estimated_tokens(),
            selection,
            excluded_sections: Vec::new(),
        })
    }
}
