mod support;

use minerva_application::{ProjectMigrationService, ProjectRepository};
use minerva_domain::{MinervaError, Project, ProjectConfig, TaskTypeDefinition};
use std::path::{Path, PathBuf};

#[test]
fn migration_service_locates_the_project_root_before_delegating() {
    struct Repo {
        root: PathBuf,
    }

    impl ProjectRepository for Repo {
        fn locate_project_root(&self, _: &Path) -> Result<PathBuf, MinervaError> {
            Ok(self.root.clone())
        }
        fn is_initialized(&self, _: &Path) -> bool {
            true
        }
        fn initialize_project(
            &self,
            _: &Path,
            _: bool,
        ) -> Result<Project, MinervaError> {
            unreachable!()
        }
        fn load_project(&self, _: &Path) -> Result<Project, MinervaError> {
            unreachable!()
        }
        fn load_project_config(&self, _: &Path) -> Result<ProjectConfig, MinervaError> {
            unreachable!()
        }
        fn load_task_types(
            &self,
            _: &Path,
        ) -> Result<Vec<TaskTypeDefinition>, MinervaError> {
            unreachable!()
        }
        fn save_project(&self, _: &Path, _: &Project) -> Result<(), MinervaError> {
            unreachable!()
        }
        fn read_project_instructions(&self, _: &Path) -> Result<String, MinervaError> {
            unreachable!()
        }
        fn write_project_instructions(
            &self,
            _: &Path,
            _: &str,
        ) -> Result<(), MinervaError> {
            unreachable!()
        }
        fn prepare_project_instructions(
            &self,
            _: &Path,
        ) -> Result<PathBuf, MinervaError> {
            unreachable!()
        }
        fn migrate_project_state(
            &self,
            root: &Path,
            dry_run: bool,
        ) -> Result<minerva_application::ProjectMigrationResult, MinervaError> {
            assert_eq!(root, self.root);
            assert!(dry_run);
            Ok(minerva_application::ProjectMigrationResult {
                start_version: 1,
                target_version: 1,
                steps: Vec::new(),
            })
        }
    }

    let repo = Repo { root: PathBuf::from("/tmp/minerva") };
    let result = ProjectMigrationService::run(&repo, Path::new("."), true).unwrap();
    assert!(result.is_current());
}
