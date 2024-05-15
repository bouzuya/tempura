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
fn test_current_directory_not_found() -> anyhow::Result<()> {
    // I can't test
    Ok(())
}

#[test]
fn test_input_is_not_json() -> anyhow::Result<()> {
    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    let tmpl_dir = temp_dir.join("tmpl");
    fs::create_dir_all(tmpl_dir.as_path())?;
    fs::write(tmpl_dir.join("{{name}}.txt"), r#"Hello,{{name}}"#)?;
    Command::cargo_bin("tempura")?
        .arg("tmpl")
        .current_dir(temp_dir)
        .write_stdin(r#"name=World"#)
        .assert()
        .failure()
        .stderr("Error: InputIsNotValidJson\n");
    Ok(())
}

#[test]
fn test_input_is_not_utf8() -> anyhow::Result<()> {
    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    let tmpl_dir = temp_dir.join("tmpl");
    fs::create_dir_all(tmpl_dir.as_path())?;
    fs::write(tmpl_dir.join("{{name}}.txt"), r#"Hello,{{name}}"#)?;
    let mut input = r#"{"name":"World"}"#.bytes().collect::<Vec<u8>>();
    assert_eq!(input[9] as char, 'W');
    assert_eq!(input[10] as char, 'o');
    // invalid UTF-8 sequence
    input[9] = 0xC2;
    input[10] = 0xCF;
    Command::cargo_bin("tempura")?
        .arg("tmpl")
        .current_dir(temp_dir)
        .write_stdin(input)
        .assert()
        .failure()
        .stderr("Error: InputIsNotUtf8\n");
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

#[test]
fn test_template_not_found() -> anyhow::Result<()> {
    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    Command::cargo_bin("tempura")?
        .arg("tmpl")
        .current_dir(temp_dir)
        .write_stdin(r#"{"name":"World"}"#)
        .assert()
        .failure()
        .stderr("Error: TemplateNotFound\n");
    Ok(())
}
