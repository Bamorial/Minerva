mod support;

use std::fs;
use support::{run, temp_dir, write_editor};

#[test]
fn instruction_command_opens_project_instructions_in_fake_editor() {
    let root = temp_dir("cli-instruction-edit");
    assert!(run(&root, &["init"]).status.success());
    let editor =
        write_editor(&root, "fake-editor.sh", "printf '\\nedited\\n' >> \"$1\"\n");
    write_config(&root, &editor);
    let output = run(&root, &["instruction"]);
    assert!(output.status.success(), "{output:?}");
    assert!(
        fs::read_to_string(root.join(".minerva/instructions.md"))
            .unwrap()
            .contains("edited")
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn instruction_command_repairs_missing_project_instructions() {
    let root = temp_dir("cli-instruction-repair");
    assert!(run(&root, &["init"]).status.success());
    fs::remove_file(root.join(".minerva/instructions.md")).unwrap();
    let editor =
        write_editor(&root, "fake-editor.sh", "printf '\\nrepaired\\n' >> \"$1\"\n");
    write_config(&root, &editor);
    assert!(run(&root, &["instruction"]).status.success());
    let contents = fs::read_to_string(root.join(".minerva/instructions.md")).unwrap();
    assert!(contents.starts_with("# Project Instructions"));
    assert!(contents.contains("repaired"));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn instruction_command_preserves_existing_contents_when_editor_fails() {
    let root = temp_dir("cli-instruction-fail");
    assert!(run(&root, &["init"]).status.success());
    fs::write(root.join(".minerva/instructions.md"), "keep me\n").unwrap();
    let editor = write_editor(&root, "fake-editor.sh", "exit 7\n");
    write_config(&root, &editor);
    let output = run(&root, &["instruction"]);
    assert!(!output.status.success(), "{output:?}");
    assert_eq!(
        fs::read_to_string(root.join(".minerva/instructions.md")).unwrap(),
        "keep me\n"
    );
    fs::remove_dir_all(root).unwrap();
}

fn write_config(root: &std::path::Path, editor: &std::path::Path) {
    fs::write(
        root.join(".minerva/config.yaml"),
        format!(
            "schema_version: 1\neditor: {}\ndefault_priority: Medium\ndefault_tags: []\n",
            editor.display()
        ),
    )
    .unwrap();
}
