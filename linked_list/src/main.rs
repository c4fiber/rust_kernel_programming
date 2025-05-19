use std::cell::RefCell;
use std::rc::Rc;

// NodeType을 Option<Rc<RefCell<Node>>>로 정의해, node의 이전 노드와 다음 노드를 참조하는 타입을 나타냅니다.
// Rc와 RefCell을 사용하면 노드를 가변으로 공유할 수 있으며 None을 사용하면 마지막 노드를 표현할 수 있습니다.
type NodeType = Option<Rc<RefCell<Node>>>;

struct Node {
    item: i32,
    prev: NodeType,
    next: NodeType,
}

impl Node {
    fn new(item: i32) -> Self {
        Self {
            item,
            prev: None,
            next: None,
        }
    }

}

pub struct DoubleLinkedList {
    head: NodeType,
    tail: NodeType,
}

impl DoubleLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, item: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(item)));
    
        if let Some(tail) = self.tail.take() {
            // tail에 값이 있으면(None이 아니면) 그 뒤에 new_node를 추가합니다.
            tail.borrow_mut().next = Some(Rc::clone(&new_node));
            new_node.borrow_mut().prev = Some(tail);
            self.tail = Some(new_node)
        } else {
            // tail에 값이 없으면(처음 삽입하는 경우) head와 tail 모두 new_node를 가리킵니다.
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        }
    }

    fn push_front(&mut self, item: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(item)));

        if let Some(head) = self.head.take() {
            head.borrow_mut().prev = Some(Rc::clone(&new_node));
            new_node.borrow_mut().next = Some(head);
            self.head = Some(new_node);
        } else {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        }
    }

    fn print_all(&mut self) {
        let mut current = match &self.head {
            Some(node) => { node.clone() },
            None => { return; }
        };

        loop {
            let t = current.clone();
            println!("item: {}", t.borrow().item);
            current = match &(t.borrow().next) {
                Some(node) => { node },
                None => { break; }
            }.clone()
        }
    }
}

fn main() {
    let mut list = DoubleLinkedList::new();

    println!("뒤에 1,2,3 삽입");
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    list.print_all();

    println!("맨 앞에 0 추가");
    list.push_front(0);
    list.print_all();
}