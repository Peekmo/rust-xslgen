pub enum IOType {
    File,
    StdIO
}

pub struct Process {
    pub input_arg: String,
    pub output_type: IOType,
    pub output_arg: Option<String>
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


