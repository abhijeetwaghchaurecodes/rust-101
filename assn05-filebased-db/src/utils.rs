
use std::io::{Write, BufWriter};

pub fn append_to_file(
    writer: &mut BufWriter<std::fs::File>,
    key: &str,
    value: &str,
) -> std::io::Result<()> {
    // Simple CSV-like record: key,value\n
    writeln!(writer, "{},{}", key, value)?;
    writer.flush()?;
    Ok(())
}
