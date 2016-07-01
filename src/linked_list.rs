use node::*;

#[derive(Debug, PartialEq)]
pub struct LinkedList {
    pub head: Node,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList {
            head: Node::new(),
        }
    }
}

#[test]
fn linked_list_intializes() {
    let new_list = LinkedList::new();

    assert_eq!(None, new_list.head.data);
    assert_eq!(None, new_list.head.next_node);
}