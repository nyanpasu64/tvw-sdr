use std::env::args;
use std::fs::File;
use std::io;
use std::process::exit;

static HELP_TEXT: &str = "{dsp, microcode}";

fn main() {
    let mut args = args();
    if args.len() <= 1 {
        eprintln!("Pass in one or more arguments out of {}", HELP_TEXT);
        exit(1);
    }

    let mut exit_code = 0;

    // Drop the program name.
    args.next();

    // Process the remaining arguments.
    for arg in args {
        match arg.as_str() {
            "dsp" => extract_dsp(),
            // "microcode" => extract_microcode(),
            _ => {
                eprintln!(
                    "Unrecognized argument \"{}\", supported = {}",
                    arg, HELP_TEXT
                );
                exit_code = 1;
                continue;
            }
        }
    }
    exit(exit_code);
}

fn extract_dsp() {
    // implicit
    use std::io::BufRead;

    let file = File::open("CTRLT507.s3").expect("could not open CTRLT507.s3");
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        let line: String = line.expect("Error reading CTRLT507.s3");
        if line.is_empty() {
            continue;
        }
        let mut line = line.as_bytes();

        if !line.starts_with(b"S3") {
            panic!("Line \"{:?}\" does not start with \"S3\"!", line);
        }
        line = &line[b"S3".len()..];


        // line
    }
}
