use std::io;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("숫자를 입력해 주세요.");
    let mut read = String::new();
    io::stdin().read_line(&mut read).unwrap();
    let index: i32 = read.trim().parse().unwrap();

    println!("arr[{}] = {}", index, arr[index as usize]);
}
