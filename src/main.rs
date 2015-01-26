#![allow(unstable)]
extern crate core;

use std::os;
use core::ops::Index;
use std::string::String;
use std::vec::Vec;

mod process;
mod parser;
mod XSL;

fn main() {
    let mut args = os::args();
    args.remove(0);

    let mut process = process::Process::new();

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
                process.output_type = process::IOType::File;
                process.output_arg  = Some(String::from_str(argument.index(&1).clone()));
            }
            _ => panic!("Unknown argument {}", argument.index(&0))
        }
    }

    match process.validate() {
        Ok(_) => {},
        Err(err) => panic!(err)
    }

    let lines = match process.read() {
        Err(err) => panic!(err.desc),
        Ok (file_lines) => file_lines
    };

    let mut parser = parser::Parser::new(lines);
    parser.parse();

    let string = XSL::build_from_nodes(&parser.nodes);
    println!("{}", string);
}
