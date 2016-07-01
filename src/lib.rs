#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Node {
    data: Option<String>,  
    next_node: Option<Box<Node>>, 
}

#[allow(dead_code)]
impl Node {
    fn new() -> Node {
        Node {
            data: None, 
            next_node: None,
        }
    } 
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct LinkedList {
    head: Node,
}

#[allow(dead_code)]
impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {
            head: Node::new(),
        }
    }
}

#[test]
fn it_works() {
    let new_list = LinkedList::new();

    assert_eq!(None, new_list.head.data);
    assert_eq!(None, new_list.head.next_node);
}