use std::{
    fmt::Display,
    fs::File,
    io::{Result, Write},
};

pub trait ToFile {
    fn to_file(&self, filename: &str) -> Result<()>;
}

impl<T: Display> ToFile for T {
    fn to_file(&self, path: &str) -> Result<()> {
        let mut file = File::create(path)?;
        file.write_fmt(format_args!("{}", self))?;
        Ok(())
    }
}
