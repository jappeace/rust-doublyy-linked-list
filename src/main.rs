
use std::sync::OnceLock;

struct DoublyLinked<'a, Element> { 
    head : &'a DoublyLinked<'a, Element>,
    tail : &'a DoublyLinked<'a, Element>,
    val: Element
}

fn singleton<'x, Element>(val: &'x Element) -> &DoublyLinked<'x, Element> {
    static fixedPoint: OnceLock<DoublyLinked<'x,&'x Element>> = OnceLock::new();
    fixedPoint.get_or_init(|| {
      DoublyLinked {
          head: &fixedPoint.get().unwrap(),
          tail: &fixedPoint.get().unwrap(),
          val: val,
      }
    })
}

fn main() {
    println!("Hello, world!");
}
