use std::{io::Write};
pub fn file_write(buf: &[u8]) -> Result<&'static str, std::io::Error> {
    let path = "result.ppm";
    let mut file = std::fs::File::create(path).expect("create failed");
    match file.write_all(buf) {
        Ok(ret) => Result::Ok(path),
        Err(e) => {
            print!("file write error message:{}", e.to_string());
            Err(e)
        }
    }
}
