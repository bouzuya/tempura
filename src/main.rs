use std::{collections::HashMap, io::Read, path::PathBuf};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("current directory not found")]
    CurrentDirectoryNotFound,
    #[error("input is not valid json")]
    InputIsNotValidJson,
    #[error("input is not UTF-8")]
    InputIsNotUtf8,
    #[error("no arguments")]
    NoArguments,
    #[error("subdirectories are not supported yet")]
    SubDirNotSupportedYet,
    #[error("template is not directory")]
    TemplateIsNotDirectory,
    #[error("template not found")]
    TemplateNotFound,
}

fn main() -> Result<(), Error> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        return Err(Error::NoArguments);
    }

    let template = PathBuf::from(args[1].as_str())
        .canonicalize()
        .map_err(|_| Error::TemplateNotFound)?;
    if !template.is_dir() {
        return Err(Error::TemplateIsNotDirectory);
    }
    let template_dir = template;
    // println!("DEBUG: template_dir = {:?}", template_dir);

    let output_dir = std::env::current_dir().map_err(|_| Error::CurrentDirectoryNotFound)?;
    // println!("DEBUG: output_dir = {:?}", output_dir);

    let mut data = String::new();
    std::io::stdin()
        .read_to_string(&mut data)
        .map_err(|_| Error::InputIsNotUtf8)?;
    let data = serde_json::from_str::<HashMap<String, String>>(data.as_str())
        .map_err(|_| Error::InputIsNotValidJson)?;

    for dir_entry in template_dir.read_dir().expect("FIXME") {
        let dir_entry = dir_entry.expect("FIXME");
        if dir_entry.file_type().expect("FIXME").is_dir() {
            return Err(Error::SubDirNotSupportedYet);
        }
        let template_file_path = dir_entry.path();
        let relative_path = template_file_path
            .strip_prefix(&template_dir)
            .expect("FIXME");
        let output_file_path = output_dir.join(
            relative_path
                .to_str()
                .expect("FIXME")
                .replace("{{name}}", data.get(&"name".to_string()).expect("FIXME")),
        );

        let template_file_content =
            std::fs::read_to_string(template_file_path.as_path()).expect("FIXME");
        let output_file_content = template_file_content
            .replace("{{name}}", data.get(&"name".to_string()).expect("FIXME"));
        println!("DEBUG: template_file_path = {:?}", template_file_path);
        println!("DEBUG: output_file_path = {:?}", output_file_path);
        println!("DEBUG: output_file_content = {:?}", output_file_content);

        std::fs::write(output_file_path, output_file_content).expect("FIXME");
    }

    Ok(())
}
