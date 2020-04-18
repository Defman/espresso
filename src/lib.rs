use std::io::Read;
use anyhow::{Result};

mod buf_ext;
mod class;

use buf_ext::TryReadFrom;

pub fn parse_class(reader: &mut impl Read) -> Result<class::ClassFile> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(class::ClassFile::try_read(&mut &buffer[..])?)
}