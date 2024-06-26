use std::fs;

use assert_cmd::Command;
use tempdir::TempDir;

#[test]
fn test_error_create_directory_failed() -> anyhow::Result<()> {
    // I can't test
    Ok(())
}

#[test]
fn test_error_create_file_failed() -> anyhow::Result<()> {
    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    let tmpl_dir = temp_dir.join("tmpl");
    fs::create_dir_all(tmpl_dir.as_path())?;
    fs::write(tmpl_dir.join("{{name}}.txt"), r#"Hello,{{name}}"#)?;
    fs::write(temp_dir.join("World.txt"), r#"already exists"#)?;
    Command::cargo_bin("tempura")?
        .arg("tmpl")
        .current_dir(temp_dir)
        .write_stdin(r#"{"name":"World"}"#)
        .assert()
        .failure()
        .stderr(predicates::str::starts_with("Error: CreateFileFailed("));
    assert_eq!(
        fs::read_to_string(temp_dir.join("World.txt"))?,
        "already exists"
    );
    Ok(())
}

#[test]
fn test_error_current_directory_not_found() -> anyhow::Result<()> {
    // I can't test
    Ok(())
}

#[test]
fn test_error_input_is_not_utf8() -> anyhow::Result<()> {
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
fn test_error_input_is_not_valid_json() -> anyhow::Result<()> {
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
fn test_error_no_arguments() -> anyhow::Result<()> {
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
fn test_error_read_directory_failed() -> anyhow::Result<()> {
    // I can't test
    Ok(())
}

#[test]
fn test_error_read_file_failed() -> anyhow::Result<()> {
    // I can't test
    Ok(())
}

#[test]
fn test_error_template_file_name_is_not_utf8() -> anyhow::Result<()> {
    // I can't test
    Ok(())
}

#[test]
fn test_error_template_is_not_directory() -> anyhow::Result<()> {
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
fn test_error_template_not_found() -> anyhow::Result<()> {
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

#[test]
fn test_error_variable_contains_path_separator() -> anyhow::Result<()> {
    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    // <temp_dir>/tmpl/{{name}}.txt
    let tmpl_dir = temp_dir.join("tmpl");
    fs::create_dir_all(tmpl_dir.as_path())?;
    fs::write(tmpl_dir.join("{{name}}.txt"), r#"Hello,{{name}}"#)?;
    Command::cargo_bin("tempura")?
        .arg("tmpl")
        .current_dir(temp_dir)
        .write_stdin(r#"{"name":"foo/bar"}"#)
        .assert()
        .failure()
        .stderr("Error: VariableContainsPathSeparator(\"{{name}}.txt\", \"foo/bar.txt\")\n");

    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    // <temp_dir>/tmpl/nested/{{name}}.txt
    let tmpl_dir = temp_dir.join("tmpl");
    fs::create_dir_all(tmpl_dir.as_path())?;
    let nested_dir = tmpl_dir.join("nested");
    fs::create_dir_all(nested_dir.as_path())?;
    fs::write(nested_dir.join("{{name}}.txt"), r#"Hello,{{name}}"#)?;
    Command::cargo_bin("tempura")?
        .arg("tmpl")
        .current_dir(temp_dir)
        .write_stdin(r#"{"name":"foo/bar"}"#)
        .assert()
        .failure()
        .stderr("Error: VariableContainsPathSeparator(\"nested/{{name}}.txt\", \"nested/foo/bar.txt\")\n");
    Ok(())
}

#[test]
fn test_error_variable_not_found() -> anyhow::Result<()> {
    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    let tmpl_dir = temp_dir.join("tmpl");
    fs::create_dir_all(tmpl_dir.as_path())?;
    fs::write(tmpl_dir.join("{{name}}.txt"), r#"Hello,{{name}}"#)?;
    Command::cargo_bin("tempura")?
        .arg("tmpl")
        .current_dir(temp_dir)
        .write_stdin(r#"{"name1":"World"}"#)
        .assert()
        .failure()
        .stderr("Error: VariableNotFound(\"name\")\n");
    Ok(())
}

#[test]
fn test_error_write_file_failed() -> anyhow::Result<()> {
    // I can't test
    Ok(())
}

#[test]
fn test_example_simple() -> anyhow::Result<()> {
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
fn test_example_some_vars() -> anyhow::Result<()> {
    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    let tmpl_dir = temp_dir.join("tmpl");
    fs::create_dir_all(tmpl_dir.as_path())?;
    fs::write(
        tmpl_dir.join("{{file}}{{date}}.txt"),
        r#"{{greet}},{{name}}!"#,
    )?;
    Command::cargo_bin("tempura")?
        .arg("tmpl")
        .current_dir(temp_dir)
        .write_stdin(r#"{"date":"20010203","file":"message","greet":"Hi","name":"World"}"#)
        .assert()
        .success();
    assert_eq!(
        fs::read_to_string(temp_dir.join("message20010203.txt"))?,
        "Hi,World!"
    );
    Ok(())
}

#[test]
fn test_example_escape_braces() -> anyhow::Result<()> {
    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    let tmpl_dir = temp_dir.join("tmpl");
    fs::create_dir_all(tmpl_dir.as_path())?;
    fs::write(tmpl_dir.join("file.txt"), r#"{{ or {{"{{"}}"#)?;
    Command::cargo_bin("tempura")?
        .arg("tmpl")
        .current_dir(temp_dir)
        .write_stdin(r#"{"{{":"unused","\"{{\"":"unused"}"#)
        .assert()
        .success();
    assert_eq!(fs::read_to_string(temp_dir.join("file.txt"))?, "{{ or {{");
    Ok(())
}

#[test]
fn test_example_template_nested_directory() -> anyhow::Result<()> {
    let temp_dir = TempDir::new("tempura")?;
    let temp_dir = temp_dir.path();
    // <temp_dir>/tmpl/nested/{{name}}.txt
    let tmpl_dir = temp_dir.join("tmpl");
    fs::create_dir_all(tmpl_dir.as_path())?;
    let nested_dir = tmpl_dir.join("nested");
    fs::create_dir_all(nested_dir.as_path())?;
    fs::write(nested_dir.join("{{name}}.txt"), r#"Hello,{{name}}"#)?;
    Command::cargo_bin("tempura")?
        .arg("tmpl")
        .current_dir(temp_dir)
        .write_stdin(r#"{"name":"World"}"#)
        .assert()
        .success();
    // <temp_dir>/nested/World.txt
    assert_eq!(
        fs::read_to_string(temp_dir.join("nested").join("World.txt"))?,
        "Hello,World"
    );
    Ok(())
}
