mod double_linked_list;
use double_linked_list::DoubleLinkedList;
fn main() {
    let mut list = DoubleLinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("{:?}", list.peek());
    println!("{:?}", list.get(1));
    println!("{:?}", list.pop());
    println!("{:?}", list.peek());
    println!("{:?}", list.length());
    println!("{:?}", list.add(1, 4));
    println!("{:?}", list.remove(1));
    list.reverse();
    for i in 0..list.length() {
        println!("{:?}", list.get(i as usize));
    }
    list.clear();
    println!("{:?}", list.length());
}