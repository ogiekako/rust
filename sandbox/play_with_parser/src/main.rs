use std::env;
use std::path::Path;

extern crate syntex_syntax as syntax;

use syntax::parse;
use syntax::ast;

fn main() {
    let args = env::args().nth(1).unwrap();
    let path = Path::new(&args);
    let sess = parse::ParseSess::new();

    let krate = parse::parse_crate_from_file(&path, &sess).unwrap();
    print!("{:?}", krate.module); // crate のmodule を debug dump
}
