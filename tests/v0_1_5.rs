use std::fs;

use assert_cmd::Command;
use tempdir::TempDir;

#[test]
fn test_error_template_dir_name_is_not_utf8() -> anyhow::Result<()> {
    // I can't test
    Ok(())
}

#[test]
fn test_example_template_variable_directory() -> anyhow::Result<()> {
    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    // <temp_dir>/tmpl/{{foo}}/{{bar}}/{{name}}.txt
    let tmpl_dir = temp_dir.join("tmpl");
    fs::create_dir_all(tmpl_dir.as_path())?;
    let nested_dir = tmpl_dir.join("{{foo}}").join("{{bar}}");
    fs::create_dir_all(nested_dir.as_path())?;
    fs::write(nested_dir.join("{{name}}.txt"), r#"Hello,{{name}}"#)?;
    Command::cargo_bin("tempura")?
        .arg("tmpl")
        .current_dir(temp_dir)
        .write_stdin(r#"{"foo":"123","bar":"456","name":"World"}"#)
        .assert()
        .success();
    // <temp_dir>/123/456/World.txt
    assert_eq!(
        fs::read_to_string(temp_dir.join("123").join("456").join("World.txt"))?,
        "Hello,World"
    );
    Ok(())
}
