use std::{io::Write};

pub fn file_write(path:&str, buf: &[u8]) -> Result<(), std::io::Error> {
 
    let mut file = std::fs::File::create(path).expect("create failed");
    match file.write_all(buf) {
        Ok(()) => Result::Ok(()),
        Err(e) => {
            print!("file write error message:{}", e.to_string());
            Err(e)
        }
    }

}
