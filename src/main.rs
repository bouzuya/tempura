use std::{
    collections::BTreeMap,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("current directory not found")]
    CurrentDirectoryNotFound,
    #[error("input is not UTF-8")]
    InputIsNotUtf8,
    #[error("input is not valid json")]
    InputIsNotValidJson,
    #[error("no arguments")]
    NoArguments,
    #[error("read directory failed: {0}")]
    ReadDirectoryFailed(String),
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
    let data = serde_json::from_str::<BTreeMap<String, String>>(data.as_str())
        .map_err(|_| Error::InputIsNotValidJson)?;
    // println!("DEBUG: data = {:?}", data);

    handle_directory(
        template_dir.as_path(),
        template_dir.as_path(),
        output_dir.as_path(),
        &data,
    )?;

    Ok(())
}

fn handle_directory(
    dir: &Path,
    template_dir: &Path,
    output_dir: &Path,
    data: &BTreeMap<String, String>,
) -> Result<(), Error> {
    let mut paths = dir
        .read_dir()
        .and_then(|read_dir| {
            read_dir
                .map(|dir_entry_result| dir_entry_result.map(|dir_entry| dir_entry.path()))
                .collect::<std::io::Result<Vec<PathBuf>>>()
        })
        .map_err(|_| Error::ReadDirectoryFailed(dir.display().to_string()))?;
    paths.sort();
    for path in paths {
        if path.is_dir() {
            handle_directory(&path, template_dir, output_dir, data)?;
        } else {
            handle_file(&path, template_dir, output_dir, data)?;
        }
    }
    Ok(())
}

fn handle_file(
    file: &Path,
    template_dir: &Path,
    output_dir: &Path,
    data: &BTreeMap<String, String>,
) -> Result<(), Error> {
    let relative_path = file.strip_prefix(template_dir).expect("FIXME");
    let file_name = relative_path.to_str().expect("FIXME");
    let output_file_path = output_dir.join(render(file_name, data));

    let template_file_content = std::fs::read_to_string(file).expect("FIXME");
    let output_file_content = render(&template_file_content, data);
    println!("DEBUG: file = {:?}", file);
    println!("DEBUG: output_file_path = {:?}", output_file_path);
    println!("DEBUG: output_file_content = {:?}", output_file_content);

    std::fs::create_dir_all(output_file_path.parent().expect("FIXME")).expect("FIXME");
    std::fs::write(output_file_path, output_file_content).expect("FIXME");
    Ok(())
}

fn render(tmpl: &str, data: &BTreeMap<String, String>) -> String {
    parse_tmpl(tmpl)
        .into_iter()
        .fold(String::new(), |acc, token| match token {
            Token::Val(val) => acc + &val,
            Token::Var(var) => acc + data.get(&var).expect("FIXME"),
        })
}

#[derive(Debug, PartialEq)]
enum Token {
    Val(String),
    Var(String),
}

fn parse_tmpl(s: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars = s.chars();
    let mut val = String::new();
    while let Some(c) = chars.next() {
        if c == '{' {
            if let Some(c_next) = chars.next() {
                if c_next == '{' {
                    val = parse_tmpl_sub(&mut chars, &mut tokens, val);
                } else {
                    val.push(c);
                    val.push(c_next);
                }
            } else {
                val.push(c);
            }
        } else {
            val.push(c);
        }
    }
    if !val.is_empty() {
        tokens.push(Token::Val(val));
    }
    tokens
}

fn parse_tmpl_sub(chars: &mut std::str::Chars, tokens: &mut Vec<Token>, mut val: String) -> String {
    let mut var = String::new();
    loop {
        match chars.next() {
            Some('}') => match chars.next() {
                Some('}') => {
                    if var.is_empty() {
                        val.push_str("{{}}");
                        break val;
                    } else if val.is_empty() {
                        tokens.push(Token::Var(var));
                        break val;
                    } else {
                        tokens.push(Token::Val(val));
                        tokens.push(Token::Var(var));
                        return String::new();
                    }
                }
                others => {
                    val.push_str("{{");
                    val.push_str(var.as_str());
                    val.push('}');
                    if let Some(c) = others {
                        val.push(c);
                    }
                    break val;
                }
            },
            Some(c) if c.is_ascii_alphanumeric() || c == '_' => {
                var.push(c);
            }
            others => {
                if var.is_empty() && others == Some('"') && {
                    let mut cs = chars.clone();
                    cs.next() == Some('{')
                        && cs.next() == Some('{')
                        && cs.next() == Some('"')
                        && cs.next() == Some('}')
                        && cs.next() == Some('}')
                } {
                    chars.next(); // {
                    chars.next(); // {
                    chars.next(); // "
                    chars.next(); // }
                    chars.next(); // }
                    val.push_str("{{");
                    break val;
                } else {
                    val.push_str("{{");
                    val.push_str(var.as_str());
                    if let Some(c) = others {
                        val.push(c);
                    }
                    break val;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tmpl() {
        use Token::*;
        let f = parse_tmpl;
        let l = |s: &str| -> Token { Val(s.to_string()) };
        let r = |s: &str| -> Token { Var(s.to_string()) };
        assert_eq!(f(""), vec![]);
        assert_eq!(
            f("ab{{cd}}ef{{gh}}"),
            vec![l("ab"), r("cd"), l("ef"), r("gh")]
        );
        assert_eq!(f("a"), vec![l("a")]);
        assert_eq!(f("a{"), vec![l("a{")]);
        assert_eq!(f("a{b"), vec![l("a{b")]);
        assert_eq!(f("a{{"), vec![l("a{{")]);
        assert_eq!(f("a{{b"), vec![l("a{{b")]);
        assert_eq!(f("a{{b}"), vec![l("a{{b}")]);
        assert_eq!(f("a{{b}c"), vec![l("a{{b}c")]);
        assert_eq!(f("a{{b}}"), vec![l("a"), r("b")]);
        assert_eq!(f("a{{b}}c"), vec![l("a"), r("b"), l("c")]);
        assert_eq!(f("{{a}}"), vec![r("a")]);
        assert_eq!(f("{{a}}b"), vec![r("a"), l("b")]);
        assert_eq!(f("{{a}}{"), vec![r("a"), l("{")]);
        assert_eq!(f("{{a}}{b"), vec![r("a"), l("{b")]);
        assert_eq!(f("{{a}}{{"), vec![r("a"), l("{{")]);
        assert_eq!(f("{{a}}{{b"), vec![r("a"), l("{{b")]);
        assert_eq!(f("{{a}}{{b}"), vec![r("a"), l("{{b}")]);
        assert_eq!(f("{{a}}{{b}c"), vec![r("a"), l("{{b}c")]);
        assert_eq!(f("{{a}}{{b}}"), vec![r("a"), r("b")]);
        assert_eq!(f("{{a}}{{b}}c"), vec![r("a"), r("b"), l("c")]);
        // escape {{
        assert_eq!(f(r#"{{""#), vec![l(r#"{{""#)]);
        assert_eq!(f(r#"{{"{"#), vec![l(r#"{{"{"#)]);
        assert_eq!(f(r#"{{"{{"#), vec![l(r#"{{"{{"#)]);
        assert_eq!(f(r#"{{"{{""#), vec![l(r#"{{"{{""#)]);
        assert_eq!(f(r#"{{"{{"}"#), vec![l(r#"{{"{{"}"#)]);
        assert_eq!(f(r#"{{"{{"}}"#), vec![l(r#"{{"#)]);
        assert_eq!(f(r#"{{a"{{"}}"#), vec![l(r#"{{a"{{"}}"#)]);
        // space is not allowed
        assert_eq!(f(r#"{{ a }}"#), vec![l(r#"{{ a }}"#)]);
        assert_eq!(f(r#"{{ "{{" }}"#), vec![l(r#"{{ "{{" }}"#)]);
    }
}
