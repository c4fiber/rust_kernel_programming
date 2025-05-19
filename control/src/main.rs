fn main() {
    // if
    let condition = true;
    let ret = if condition == true {
        String::from("조건이 참입니다.")
    } else {
        String::from("조건이 거짓입니다.")
    };
    println!("ret={}", ret);

    // match
    let var = 1;

    match var {
        1 => println!("var는 1입니다."),
        2 => println!("var는 2입니다."),
        _ => println!("var는 1도 아니고 2도 아닙니다."),
    }

    let ret = match var {
        1 => String::from("var는 1입니다."),
        2 => String::from("var는 2입니다."),
        _ => String::from("var는 1도 아니고 2도 아닙니다."),
    };
    println!("ret={}", ret);

    // for
    let arr = [1, 2, 3, 4, 5];

    for a in arr {
        print!("{}, ", a);
    }

    for a in 0..5 {
        print!("{}, ", a);
    }

    // while
    let mut counter = 0;

    while counter < 5 {
        println!("counter={}", counter);
        counter += 1;
    }

    // loop
    loop {
        println!("숫자를 입력해 주세요. 0을 입력하면 종료합니다.");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let val: i32 = input.trim().parse().unwrap();

        if val == 0 {
            println!("종료합니다.");
            break;
        }

        println!("입력한 숫자는 {}입니다.", val);
    }

    // function
    let ret = add(1, 2);
    println!("ret={}", ret);

    //
    let x = 1;
    let y = 2;
    let ret = { x + y };

    println!("{}+{}={}", x, y, ret);

    // closure
    let mut x = 5;
    let mut inc = || {
        x += 1;
    };
    inc();
    println!("x={}", x);

    // closure with parameter
    let x = 10;
    let add = |y| x + y;
    println!("10 + 5 = {}", add(5));

    let s = String::from("Hello");
    let f = move || {
        println!("s: {}", s);
    };

    f();
    // println!("s: {}", s); // This will cause an error because s has been moved into the closure
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

