#![allow(unstable)]
use std::io::fs::File;
use std::path::posix::Path;
use std::io::IoError;
use std::vec::Vec;

/// Types of IO for output_type of `Process`
#[stable]
pub enum IOType {
    File,
    StdIO
}

/// Process struct contains
/// input_arg => Path to the file to parse
/// output_type => Type of the output (file or stdout @see `IOType`)
/// output_arg => If the output is a file, the filename is stored here
#[unstable]
pub struct Process {
    pub input_arg: String,
    pub output_type: IOType,
    pub output_arg: Option<String>
}

impl Process {
    /// Builds a new Process with empty file as input, output to standard output
    #[stable]
    pub fn new() -> Self {
        Process {
            input_arg: String::from_str(""),
            output_type: IOType::StdIO,
            output_arg: None
        }
    }

    /// Validation of the `Process` struct data
    ///
    /// # Errors
    /// A `String` with the error will be returned if needed
    #[unstable]
    pub fn validate(&self) -> Result<(), String> {
        if self.input_arg == "" {
            return Err(String::from_str("No input file found (option --file=path/to/file.xslg)"));
        }

        return Ok(());
    }

    /// Read the content of the file from input_arg and returns a `Vec<String>`
    /// that contains the XSLG file line by line (it removes empty lines).
    ///
    /// # Errors
    /// Errors can be return if there's a problem to read the file
    #[stable]
    pub fn read(&self) -> Result<Box<Vec<String>>, IoError> {
        let mut file = try!(File::open(&Path::new(self.input_arg.as_slice())));
        let content = try!(file.read_to_string());

        let mut it = content.as_slice().split('\n');
        let mut lines = Vec::new();
        loop {
            match it.next() {
                None => break,
                Some(ref line) => {
                    // Remove empty lines & comment lines
                    if line.trim() != "" && !line.trim().starts_with("#") {
                        lines.push(String::from_str(line.trim()));
                    }
                }
            }
        }

        return Ok(Box::new(lines));
    }
}


