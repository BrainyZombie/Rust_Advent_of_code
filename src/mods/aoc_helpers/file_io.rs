use std::{env::Args, fs};
pub fn file_io<F>(mut args: Args, f: F) -> Result<String, String>
where
    F: FnOnce(&str, Args) -> Result<String, String>,
{
    let input_file_path = match args.next() {
        Some(s) => s,
        None => return Err(String::from("No input file path specified!")),
    };
    let output_file_path = match args.next() {
        Some(s) => s,
        None => return Err(String::from("No output file path specified!")),
    };
    let input_content = match fs::read_to_string(input_file_path) {
        Ok(s) => s,
        Err(e) => return Err(format!("Read file error: {e}")),
    };
    f(input_content.as_str(), args)
        .map_or_else(Err, |output| {
            Ok((fs::write(output_file_path, &output), output))
        })
        .map_or_else(Err, |(write_result, output)| {
            if let Err(e) = write_result {
                Err(format!("Write file error: {e}"))
            } else {
                Ok(output)
            }
        })
}
