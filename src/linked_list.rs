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

    pub fn tail(&mut self, node: Option<Box<Node>>) -> &mut Node {
        match node {
            Some(f) => {
                if f.next_node == None {
                    let current_node = &mut self.head;
                    current_node
                } else {
                    self.tail(f.next_node)
                }
            },
            None => {
                self.tail(Some(Box::new(Node::new())))
            }
        }
    }
}

#[test]
fn linked_list_intializes() {
    let new_list = LinkedList::new();

    assert_eq!(None, new_list.head.data);
    assert_eq!(None, new_list.head.next_node);
}

#[test]
fn it_can_find_tail() {
    let mut new_list = LinkedList::new();
    let mut test_list = LinkedList::new();

    assert_eq!(None, new_list.head.next_node);
    assert_eq!(&mut test_list.head, new_list.tail(None));
}