
use std::io::Write;
pub fn file_write(mut buf: &[u8]){
    
let mut file =std::fs::File::create("result.ppm").expect("create failed");
file.write_all(buf).expect("write failed");

}