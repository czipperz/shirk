use diagnostics;

pub struct Arguments {
    pub filename: String,
    pub dump_tokens: bool,
    pub dump_syntax_tree: bool,
}

pub fn parse_arguments(mut args: Vec<String>) -> Option<Arguments> {
    let mut filename = None;
    let mut dump_tokens = false;
    let mut dump_syntax_tree = false;

    if args.len() >= 1 { args.remove(0); }
    for arg in args {
        if arg == "-compiler-dump=tokens" {
            dump_tokens = true;
            continue;
        } else if arg == "-compiler-dump=syntax" {
            dump_syntax_tree = true;
            continue;
        } else {
            match arg.chars().next() {
                Some('-') => {
                    diagnostics::print_error(
                        format_args!("Unknown option {}", arg));
                    return None;
                },
                _ => (),
            };
        }

        if filename.is_some() {
            diagnostics::print_error(
                format_args!("Already specified file to compile"));
            return None;
        }
        filename = Some(arg);
    }

    match filename {
        Some(filename) =>
            Some(Arguments { filename: filename,
                             dump_tokens: dump_tokens,
                             dump_syntax_tree: dump_syntax_tree }),
        None => {
            diagnostics::print_error(
                format_args!("Did not specify a file to compile"));
            None
        }
    }
}
