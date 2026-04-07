mod linked_list;
use linked_list::LinkedList;
fn main() {
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("{:?}", list.peek());
    println!("{:?}", list.get(1));
    println!("{:?}", list.pop());
    println!("{:?}", list.length());
}