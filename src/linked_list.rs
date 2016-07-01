use node::*;

#[derive(Debug, PartialEq)]
pub struct LinkedList {
    pub head: Node,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList { head: Node::new() }
    }

    pub fn count(&mut self) -> usize {
        let mut count = 0;
        let mut current_node = &self.head;

        while let Some(ref f) = current_node.next_node {
            current_node = f;
            count += 1;
        }

        count
    }

    pub fn append(&mut self, data: Option<String>) {
        let mut new_node = Node::new();
        new_node.data = data;
        self.head.tail().next_node = Some(Box::new(new_node));
    }

    pub fn tail(&mut self) -> &mut Node {
        self.head.tail()
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

    new_list.head.data = Some("hello".to_string());

    assert_eq!(None, new_list.head.next_node);
    assert_eq!(Some("hello".to_string()), new_list.tail().data);
}

#[test]
fn it_can_append_data() {
    let mut new_list = LinkedList::new();

    new_list.append(Some("wow".to_string()));

    assert_eq!(Some("wow".to_string()), new_list.tail().data)
}

#[test]
fn it_can_count_the_amount_of_nodes() {
    let mut new_list = LinkedList::new();

    new_list.append(Some("wow1".to_string()));
    new_list.append(Some("wow2".to_string()));
    new_list.append(Some("wow3".to_string()));
    new_list.append(Some("wow4".to_string()));

    assert_eq!(4, new_list.count())
}