
struct DoublyLinked<'a, Element> { 
    head : Box<dyn Fn() -> DoublyLinked<'a, Element> + 'a>,
    tail : Box<dyn Fn() -> DoublyLinked<'a, Element> + 'a>,
    val: Element
}

fn singleton<'x, Element>(val: &'x Element) -> DoublyLinked<'x, &'x Element> {
    let mut begin = DoublyLinked { 
        head: Box::new(|| panic!("Head not set")),
        tail: Box::new(|| panic!("Tail not set")),
        val: val
    };

    begin.head = Box::new(|| begin); 
    begin.tail = Box::new(|| begin); 

    begin
}

fn main() {
    println!("Hello, world!");
}
