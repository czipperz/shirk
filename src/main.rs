mod arguments;
mod diagnostics;
mod file_position;
mod lex;
mod parse;

use std::env;
use std::process;
use std::fs;
use std::path::Path;
use std::io::Read;
use std::rc::Rc;

fn main() {
    let args: arguments::Arguments =
        match arguments::parse_arguments(env::args().collect()) {
            Some(args) => args,
            None => process::exit(1),
        };

    let file: fs::File =
        match fs::File::open(Path::new(&args.filename)) {
            Ok(file) => file,
            Err(e) => {
                diagnostics::print_error(
                    format_args!("Cannot open file: {} -- {}",
                                 args.filename, e));
                process::exit(1);
            },
        };

    let (tokens, success) =
        lex::lex(lex::FileIterator {
            bytes: file.bytes(),
            fpos: file_position::FilePosition {
                filename: Rc::new(args.filename),
                line: 1,
                column: 0,
            },
        });
    if args.dump_tokens {
        for token in &tokens {
            diagnostics::print_fpos(&token.fpos);
            println!("{}", token.data);
        }
    }
    if !success {
        process::exit(1);
    }

    let toplevels = match parse::parse(tokens.into_iter()) {
        Ok(toplevels) => toplevels,
        Err(()) => {
            process::exit(1);
        },
    };
    if args.dump_syntax_tree {
        for toplevel in &toplevels {
            println!("{:?}", toplevel);
        }
    }
}
