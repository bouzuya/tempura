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
        let file_name = relative_path.to_str().expect("FIXME");
        let output_file_path = output_dir.join(render(file_name, &data));

        let template_file_content =
            std::fs::read_to_string(template_file_path.as_path()).expect("FIXME");
        let output_file_content = render(&template_file_content, &data);
        println!("DEBUG: template_file_path = {:?}", template_file_path);
        println!("DEBUG: output_file_path = {:?}", output_file_path);
        println!("DEBUG: output_file_content = {:?}", output_file_content);

        std::fs::write(output_file_path, output_file_content).expect("FIXME");
    }

    Ok(())
}

fn render(tmpl: &str, data: &HashMap<String, String>) -> String {
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
