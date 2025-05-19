#[derive(Debug)]
struct Student {
    id: i32,
    name: String,
    email: String,
}

fn create_student(id: i32, name: String, email: String) -> Student {
    Student { id: id, name: name, email: email }
}

fn print_optional(val: Option<String>) {
    match val {
        Some(v) => println!("Value: {}", v),
        None => println!("No value"),
    }
}

fn main() {
    // enum option
    let some_number = Some(99);
    let some_string = Some("hello");
    let can_be_none: Option<i32> = None;

    let some_string = Some(String::from("rust"));
    let none_string: Option<String> = None;

    print_optional(some_string);
    print_optional(none_string);

    // enum protocol
    let m = Message::Put(String::from("/root/"));
    m.execute();

    let m = Message::List(33);
    m.execute();

    // Student struct
    println!("input student id:");
    let mut id = String::new();
    std::io::stdin().read_line(&mut id).expect("Failed to read line");
    let id: i32 = id.trim().parse().unwrap();

    println!("input student name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();

    println!("input student email:");
    let mut email = String::new();
    std::io::stdin().read_line(&mut email).expect("Failed to read line");
    let email = email.trim().to_string();

    let stu = create_student(id, name, email);
    println!("Student ID={}, Student name={}, Student email={}", stu.id, stu.name, stu.email);
    println!("stu={:?}", stu);
}

struct Score {
    score: i32,
}

impl Score {
    fn get_grade (&self) -> String {
        if self.score >= 90 {
            "A".to_string()
        } else if self.score >= 80 {
            "B".to_string()
        } else if self.score >= 70 {
            "C".to_string()
        } else if self.score >= 60 {
            "D".to_string()
        } else {
            "F".to_string()
        }
    }

    fn from(score: i32) -> Score {
        Score { score }
    }
}

#[test]
fn test_get_grade() {
    let score = Score { score: 85 };
    assert_eq!(score.get_grade(), "B".to_string());

    let score = Score { score: 95 };
    assert_eq!(score.get_grade(), "A".to_string());

    let score = Score { score: 75 };
    assert_eq!(score.get_grade(), "C".to_string());

    let score = Score { score: 55 };
    assert_eq!(score.get_grade(), "F".to_string());

    assert_eq!(Score::from(100).get_grade(), "A".to_string());
}

enum SchoolKind {
    Elementary(ElementarySchool),
    Middle(MiddleSchool),
    High(HighSchool),
}

struct ElementarySchool {
    room: String
}
struct MiddleSchool {
    teacher: String
}

struct HighSchool {
    id: i32
}

enum Message {
    Quit,
    List(i32),
    Put(String),
    Get(i32),
}

impl Message {
    fn execute(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::List(val) => println!("List: {}", val),
            Message::Put(val) => println!("Put: {}", val),
            Message::Get(val) => println!("Get: {}", val),
        }
    }
}