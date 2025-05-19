use std::cell::RefCell;
use std::rc::{Rc, Weak};

static GLOBAL_CONST: i32 = 10;

struct Person {
    id: i32,
    next: RefCell<Option<Weak<Person>>>,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Dropping Person with id: {}", self.id);
    }
}

fn main() {
    // Global constant
    let x: &'static str = "Hello Rust!";
    // x is a static string literal, which has a static lifetime.
    // It is stored in the read-only memory of the program and lives for the entire duration of the program.
    println!("x: {}", x);
    println!("GLOBAL_CONST: {}", GLOBAL_CONST);

    let p1 = Rc::new(Person {
        id: 1,
        next: RefCell::new(None),
    });
    let p2 = Rc::new(Person {
        id: 2,
        next: RefCell::new(None),
    });
    let p3 = Rc::new(Person {
        id: 3,
        next: RefCell::new(None),
    });

    let mut next = p1.next.borrow_mut();
    *next = Some(Rc::downgrade(&p2));

    let mut next = p2.next.borrow_mut();
    *next = Some(Rc::downgrade(&p3));

    println!(
        "p1 RefCount: {}, p2: RefCount: {}, p3: RefCount: {}",
        Rc::strong_count(&p1),
        Rc::strong_count(&p2),
        Rc::strong_count(&p3)
    );

    // main function이 종료되는 순간 p1, p2, p3의 RefCount가 하나씩 줄어든다.
    // p1, p2의 next는 Weak Reference이므로 RefCount에 영향을 주지 않아서 p1, p2, p3는 모두 drop된다.

    longest("hello", "world");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
