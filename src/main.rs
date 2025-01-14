use std::sync::RwLock;

#[derive(Clone)]
struct DoublyLinked<'a, Element> { 
    head : Option<&'a DoublyLinked<'a, Element>>,
    tail : Option<&'a DoublyLinked<'a, Element>>,
    val: Element
}


fn singleton<'x, Element>(val: Element) -> DoublyLinked<'x, Element> {
  DoublyLinked { 
        head: None,
        tail: None,
        val: val
  }
}
fn add<'x, Element>(tail: &'x DoublyLinked<'x, Element>, head: &'x mut DoublyLinked<'x, Element>) -> DoublyLinked<'x, Element>  where Element: Clone {
    head.tail = Some(tail);

    DoublyLinked {
        head: Some(head),
        tail: tail.tail,
        val: tail.val.clone()
    }
}

fn main() {
    println!("hello world");

}
