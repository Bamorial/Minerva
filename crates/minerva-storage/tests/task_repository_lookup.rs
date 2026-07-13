mod support;

use minerva_application::TaskRepository;
use minerva_domain::MinervaError;
use minerva_storage::FilesystemTaskRepository;
use support::{task, temp_repo};

#[test]
fn repository_lists_reads_searches_and_resolves_tasks() {
    let root = temp_repo("task-repository-lookup");
    let repo = FilesystemTaskRepository;
    let first = task(1, "Implement task repo");
    let second = task(2, "Review task repo");
    let third = task(3, "Write docs");
    repo.create_task(&root, &second).unwrap();
    repo.create_task(&root, &third).unwrap();
    repo.create_task(&root, &first).unwrap();
    assert_eq!(
        repo.list_tasks(&root).unwrap(),
        vec![first.clone(), second.clone(), third]
    );
    assert_eq!(repo.read_task(&root, first.id).unwrap(), first);
    assert_eq!(
        repo.search_tasks(&root, "review task repo").unwrap(),
        vec![second.clone()]
    );
    assert_eq!(
        repo.search_tasks(&root, "task repo").unwrap(),
        vec![first.clone(), second.clone()]
    );
    assert_eq!(
        repo.resolve_task(&root, &second.id.to_string()).unwrap(),
        second.clone()
    );
    assert_eq!(repo.resolve_task(&root, "Implement task repo").unwrap(), first);
}

#[test]
fn ambiguous_title_matches_return_a_structured_error() {
    let root = temp_repo("task-repository-ambiguous");
    let repo = FilesystemTaskRepository;
    repo.create_task(&root, &task(1, "Implement task repo")).unwrap();
    repo.create_task(&root, &task(2, "Review task repo")).unwrap();
    let error = repo.resolve_task(&root, "task repo").unwrap_err();
    assert_eq!(
        error,
        MinervaError::AmbiguousTaskReference {
            task_ref: "task repo".into(),
            matches: vec![
                "TSK-000001 Implement task repo".into(),
                "TSK-000002 Review task repo".into(),
            ],
        }
    );
}
