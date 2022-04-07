use std::env::args_os;
use std::fs::File;
use std::io::Read;
use std::process::exit;

use srec::*;

static HELP_TEXT: &str = "{dsp, microcode}";

fn main() {
    let mut args = args_os();
    if args.len() <= 1 {
        eprintln!("Pass in one or more arguments out of {}", HELP_TEXT);
        exit(1);
    }

    let mut exit_code = 0;

    // Drop the program name.
    args.next();

    // Process the remaining arguments.
    for arg in args {
        let arg = arg.as_os_str();
        if arg == "dsp" {
            extract_dsp();
        } else if arg == "microcode" {
            // extract_microcode();
        } else {
            eprintln!("Unrecognized argument {:?}, supported = {}", arg, HELP_TEXT);
            exit_code = 1;
            continue;
        }
    }
    exit(exit_code);
}

fn extract_dsp() {
    let mut file = File::open("CTRLT507.s3").expect("could not open CTRLT507.s3");
    let mut buf = Vec::<u8>::new();
    file.read_to_end(&mut buf)
        .expect("Error reading CTRLT507.s3");
    let str = std::str::from_utf8(&buf).expect("CTRLT507.s3 is not valid UTF-8");

    let mut records = srec::read_records(str);
    let record: Record = records
        .next()
        .expect("Missing data in CTRLT507.s3")
        .unwrap();

    let record = match record {
        S3(ata<Address32>)
    };
}
