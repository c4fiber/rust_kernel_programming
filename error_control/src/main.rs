use std::fs::File;
use std::io::Read;
use std::io;

fn read_from_file(path: String) -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    match f.read_to_string(&mut s) {
        Ok(_len) => return Ok(s),
        Err(e) => return Err(e),
    };
}

fn main() {
    let ret = read_from_file(String::from("test.txt")).expect("파일 읽기 중 오류가 발생했습니다.");
    println!("test.txt: {}", ret);
}