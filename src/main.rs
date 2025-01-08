
struct DoublyLinked<'a, Element> { 
    head : fn () -> &'a DoublyLinked<'a, Element>,
    tail : fn () -> &'a DoublyLinked<'a, Element>,
    val: Element
}

fn singleton<'x, Element>(val: &'x Element) -> DoublyLinked<'x, &'x Element> {
    let mut begin = DoublyLinked { 
        head: || panic!("Head not set"),
        tail: || panic!("Head not set"),
        val: val
    };

    begin.head = || &begin;
    begin.tail = || begin;

    begin
}

fn main() {
    println!("Hello, world!");
}
