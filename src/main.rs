
use std::sync::Arc;
use std::sync::RwLock;

struct DoublyLinked<'a, Element> { 
    head : Box<dyn Fn() -> Arc<RwLock<DoublyLinked<'a, Element>>> + 'a>,
    tail : Box<dyn Fn() -> Arc<RwLock<DoublyLinked<'a, Element>>> + 'a>,
    val: Element
}

fn singleton<'x, Element>(val: &'x Element) -> Arc<RwLock<DoublyLinked<'x, &'x Element>>> {
    let begin = Arc::new(RwLock::new(DoublyLinked { 
        head: Box::new(|| panic!("Head not set")),
        tail: Box::new(|| panic!("Tail not set")),
        val: val
    }));

    let begin_clone = Arc::clone(&begin);

    begin.write().unwrap().head = Box::new(move || Arc::clone(&begin_clone)); 

    let begin_clone2 = Arc::clone(&begin);
    begin.write().unwrap().tail = Box::new(move || Arc::clone(&begin_clone2)); 

    begin
}


fn add_tail<'x, Element>(me: Arc<RwLock<DoublyLinked<'x, &'x Element>>>, val: &'x Element) -> Arc<RwLock<DoublyLinked<'x, &'x Element>>> {

    let begin = singleton(val);

    let begin_clone = Arc::clone(&begin);
    me.write().unwrap().tail = Box::new(move || Arc::clone(&begin_clone)); 

    let me_clone = Arc::clone(&me);
    begin.write().unwrap().head = Box::new(move || Arc::clone(&me_clone));

    me
}

fn add_head<'x, Element>(me: Arc<RwLock<DoublyLinked<'x, &'x Element>>>, val: &'x Element) -> Arc<RwLock<DoublyLinked<'x, &'x Element>>> {

    let begin = singleton(val);

    let begin_clone = Arc::clone(&begin);
    me.write().unwrap().head = Box::new(move || Arc::clone(&begin_clone)); 

    let me_clone = Arc::clone(&me);
    begin.write().unwrap().tail = Box::new(move || Arc::clone(&me_clone));

    me
}

fn main() {
    let first = singleton(&1);

    let val = ((first.read().unwrap().tail)().read().unwrap().tail)().read().unwrap().val;

    println!("Hello, world! {0}", val);

    let second = add_tail(first, &3);

    let val3 = (second.read().unwrap().val);
    println!("Hello, world! {0}", val3);

    let val2 = (second.read().unwrap().tail)().read().unwrap().val;

    println!("Hello, world! {0}", val2);


}
