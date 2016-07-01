mod node;
mod linked_list;

use node::*;
use linked_list::*;

pub fn new_node() -> Node {
    Node::new()
}

pub fn new_list() -> LinkedList {
    LinkedList::new()
}

pub fn count_list(list: &mut LinkedList) -> usize {
    list.count()
}

pub fn tail_list<'a>(list: &'a mut LinkedList) -> &'a mut Node {
    list.tail()
}