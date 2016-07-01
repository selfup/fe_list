#[derive(Debug, PartialEq)]
pub struct Node {
    pub data: Option<String>,  
    pub next_node: Option<Box<Node>>, 
}

impl Node {
    pub fn new() -> Node {
        Node {
            data: None, 
            next_node: None,
        }
    } 
}

#[test]
fn node_intializes() {
    let new_node = Node::new();

    assert_eq!(None, new_node.data);
    assert_eq!(None, new_node.next_node);
}