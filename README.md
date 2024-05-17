# tempura

The tempura is a template engine written in Rust. It is designed to work with the file system (files and directories).

## Installation

TODO

## Usage

```console
$ ls
tmpl

$ ls tmpl/
{{name}}.txt

$ cat 'tmpl/{{name}}.txt'
Hello,{{name}}

$ echo '{"name":"World"}' | tempura tmpl

$ ls
World.txt tmpl

$ cat World.txt
Hello,World
```

### Template Syntax

You can embed variables in the template in the format `{{var_name}}`.

Variable names can use the characters `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz`.

You can use `"{{"` as a special variable name. This will be replaced with the value `{{`. For example, `{{"{{"}}` will be rendered as `{{`. Note that you cannot use `"` in variable names.

You cannot have spaces before or after `var_name`. For example, `{{ var_name }}` cannot be used.

If the variable name is invalid or contains spaces, it will be displayed as is. For example, `{{inv@lid_v@r_n@me}}` will be rendered as `{{inv@lid_v@r_n@me}}`.
