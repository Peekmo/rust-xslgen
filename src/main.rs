extern crate core;

use std::os;
use core::ops::Index;
use std::string::String;

enum IOType {
    File,
    StdIO
}

struct Process {
    input_arg: String,
    output_type: IOType,
    output_arg: Option<String>
}

impl Process {
    pub fn new() -> Self {
        Process {
            input_arg: String::from_str(""),
            output_type: IOType::StdIO,
            output_arg: None
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.input_arg == "" {
            return Err(String::from_str("No input file found"));
        }

        return Ok(());
    }
}

fn main() {
    let mut args = os::args();
    args.remove(0);

    let mut process = Process::new();

    for arg in args.iter() {
        let argument: Vec<&str> = arg.as_slice().split('=').collect();

        if argument.len() < 2 {
            panic!("Error for argument '{}'. Must be --arg=value", arg);
        }

        match *argument.index(&0) {
            "--file" => {
                process.input_arg = String::from_str(argument.index(&1).clone());
            },
            "--output" => {
                process.output_type = IOType::File;
                process.output_arg  = Some(String::from_str(argument.index(&1).clone()));
            }
            _ => panic!("Unknown argument {}", argument.index(&0))
        }
    }

    match process.validate() {
        Ok(_) => {},
        Err(err) => panic!(err)
    }
}
