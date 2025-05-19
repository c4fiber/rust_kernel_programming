use std::fs::File;

fn main() {
    let result = File::open("test.txt");
    let f = match result {
        Ok(f) => f,
        Err(err) => {
            println!("Error opening file: {:?}", err);
            return;
        },
    };

    println!("File opened successfully: {:?}", f);
}
