use std::env::args_os;
use std::fs::File;
use std::process::exit;

use srec::*;

static HELP_TEXT: &str = "{dsp, microcode}";

fn main() {
    let mut args = args_os();
    if args.len() <= 1 {
        extract_dsp();
        extract_microcode();
        return;
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
            extract_microcode();
        } else {
            eprintln!("Unrecognized argument {:?}, supported = {}", arg, HELP_TEXT);
            exit_code = 1;
            continue;
        }
    }
    exit(exit_code);
}

fn extract_dsp() {
    use std::io::{Read, Write};

    let buf = {
        let mut file = File::open("CTRLT507.s3").expect("could not open CTRLT507.s3");
        let mut buf = Vec::<u8>::new();
        file.read_to_end(&mut buf)
            .expect("Error reading CTRLT507.s3");
        buf
    };

    let str = std::str::from_utf8(&buf).expect("CTRLT507.s3 is not valid UTF-8");

    let data: Vec<u8> = {
        let mut binary = Vec::<u8>::new();

        // Each line is a record.
        let records = srec::read_records(str);

        for record in records {
            let record = record.unwrap();
            let line: Data<Address32> = match record {
                Record::S3(data) => data,
                _ => {
                    // The official driver ignores all non-S3 lines of text.
                    continue;
                }
            };
            let begin_addr = line.address.0 as usize - 0xBFC00000;
            let end_addr = begin_addr + line.data.len();

            binary.resize(binary.len().max(end_addr), 0);
            (&mut binary[begin_addr..end_addr]).copy_from_slice(&line.data);
        }

        binary
    };
    drop(buf);

    let mut of = File::create("CTRLT507.bin").expect("could not open CTRLT507.bin");
    of.write_all(data.as_ref())
        .expect("Error writing CTRLT507.bin");
    of.flush().expect("Error flushing CTRLT507.bin");
}

fn extract_microcode() {
    use std::io::{Read, Write};

    let buf = {
        let mut file = File::open("gtatinavrr.sys").expect("could not open gtatinavrr.sys");
        let mut buf = Vec::<u8>::new();
        file.read_to_end(&mut buf)
            .expect("Error reading gtatinavrr.sys");
        buf
    };

    // b"T507 AMD"
    let search: jetscii::Bytes<_> = jetscii::bytes!(0x54, 0x35, 0x30, 0x37, 0x20, 0x41, 0x4D, 0x44);

    let fw_pos = search
        .find(buf)
        .expect("Could not find \"T507 AMD\" firmware in gtatinavrr.sys");
}
