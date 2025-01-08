
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

// proper monoid
// I suppose  it's also with flipped args
fn add<'x, Element>(tail: Arc<RwLock<DoublyLinked<'x, &'x Element>>>, head: Arc<RwLock<DoublyLinked<'x, &'x Element>>>) -> Arc<RwLock<DoublyLinked<'x, &'x Element>>> {

    let head_clone = Arc::clone(&head);
    tail.write().unwrap().head = Box::new(move || Arc::clone(&head_clone)); 

    let tail_clone = Arc::clone(&tail);
    head.write().unwrap().tail = Box::new(move || Arc::clone(&tail_clone));

    tail
}


fn add_tail<'x, Element>(me: Arc<RwLock<DoublyLinked<'x, &'x Element>>>, val: &'x Element) -> Arc<RwLock<DoublyLinked<'x, &'x Element>>> {
    let begin = singleton(val);
    add(begin, me.clone());
    me
}



fn add_head<'x, Element>(me: Arc<RwLock<DoublyLinked<'x, &'x Element>>>, val: &'x Element) -> Arc<RwLock<DoublyLinked<'x, &'x Element>>> {
    let begin = singleton(val);
    add(me, begin)
}

// dop the head, setting it to the next head
fn drop_head<'x, Element>(me: Arc<RwLock<DoublyLinked<'x, &'x Element>>>) -> Arc<RwLock<DoublyLinked<'x, &'x Element>>>{

    let to_drop = ((me.read().unwrap().head)().read().unwrap().head)();

    println!("Hello, world1");
    let next_head = Arc::clone(&to_drop);

    println!("Hello, world2");
    let next_head2 = Arc::clone(&to_drop);

    println!("Hello, world3");
    me.write().unwrap().head = Box::new(move || Arc::clone(&next_head));

    println!("Hello, world4");
    let me_clone = Arc::clone(&me);

    println!("Hello, world5");
    next_head2.write().unwrap().tail = Box::new(move || Arc::clone(&me_clone));

    me
}

fn main() {
    let first = singleton(&1);

    let val = ((first.read().unwrap().tail)().read().unwrap().tail)().read().unwrap().val;

    println!("1 expect 1 {0}", val);

    let second = add_tail(first, &3);

    let val3 = (second.read().unwrap().val);
    println!("1 expect 1 {0}", val3);

    let val2 = (second.read().unwrap().tail)().read().unwrap().val;

    println!("2 expect 3 {0}", val2);


    let third = add_head(second, &9);

    let elem = third.read().unwrap(); // elem at 1

    println!("3 expect 1 {0}", elem.val);

    println!("4 expect 9 {0}", (elem.head)().read().unwrap().val);
    println!("5 expect 3 {0}", (elem.tail)().read().unwrap().val);

    println!("6 expect 1 {0}", ((elem.tail)().read().unwrap().head)().read().unwrap().val);
    println!("6 expect 9 {0}", (((elem.tail)().read().unwrap().head)().read().unwrap().head)().read().unwrap().val);

    let nineAsHead = drop_head((elem.tail)());

    let val4 = nineAsHead.read().unwrap().val;
    println!("6. expect 9 {0}", val4);

}
