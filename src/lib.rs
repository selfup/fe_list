mod node;
mod linked_list;

use node::*;
use linked_list::*;

pub fn new_fe_node() -> Node {
    Node::new()
}

pub fn new_fe_list() -> LinkedList {
    LinkedList::new()
}