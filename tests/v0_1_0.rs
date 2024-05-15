use std::fs;

use assert_cmd::Command;
use tempdir::TempDir;

#[test]
fn test_simple() -> anyhow::Result<()> {
    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    let tmpl_dir = temp_dir.join("tmpl");
    fs::create_dir_all(tmpl_dir.as_path())?;
    fs::write(tmpl_dir.join("{{name}}.txt"), r#"Hello,{{name}}"#)?;
    Command::cargo_bin("tempura")?
        .arg("tmpl")
        .current_dir(temp_dir)
        .write_stdin(r#"{"name":"World"}"#)
        .assert()
        .success();
    assert_eq!(
        fs::read_to_string(temp_dir.join("World.txt"))?,
        "Hello,World"
    );
    Ok(())
}

#[test]
fn test_no_arguments() -> anyhow::Result<()> {
    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    let tmpl_dir = temp_dir.join("tmpl");
    fs::create_dir_all(tmpl_dir.as_path())?;
    fs::write(tmpl_dir.join("{{name}}.txt"), r#"Hello,{{name}}"#)?;
    Command::cargo_bin("tempura")?
        .current_dir(temp_dir)
        .write_stdin(r#"{"name":"World"}"#)
        .assert()
        .failure()
        .stderr("Error: NoArguments\n");
    Ok(())
}

#[test]
fn test_template_is_not_directory() -> anyhow::Result<()> {
    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    let tmpl_dir = temp_dir.join("tmpl");
    fs::write(tmpl_dir, r#"temp is a file"#)?;
    Command::cargo_bin("tempura")?
        .arg("tmpl")
        .current_dir(temp_dir)
        .write_stdin(r#"{"name":"World"}"#)
        .assert()
        .failure()
        .stderr("Error: TemplateIsNotDirectory\n");
    Ok(())
}
