# tempura

The tempura is a template engine written in Rust. It is designed to work with the file system (files and directories).

## Installation

TODO

## Usage

TODO

## Examples

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
