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

    pub fn append(&mut self, data: Option<String>) {
        let mut new_node = Box::new(Node::new());
        match data {
            Some(f) => { 
                new_node.data = Some(f);
                self.tail(None).next_node = Some(new_node); 
            },
            None => { panic!("No data given!") }
        }
    }

    pub fn tail(&self, node: Option<&Node>) -> &Node {
        let current_node = &self.head;
        match node {
            Some(f) => {
                if f.next_node == None {
                    current_node
                } else {
                    self.tail(Some(current_node))
                }
            },
            None => {
                if current_node.next_node == None {
                    current_node
                } else {
                    self.tail(Some(current_node))
                }
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
    let new_list = LinkedList::new();

    assert_eq!(None, new_list.head.next_node);
    assert_eq!(&new_list.head, new_list.tail(None));
}