use std::sync::RwLock;
use std::sync::Arc;

#[derive(Clone)]
struct DoublyLinked<Element> { 
    head : Option<Arc<RwLock<DoublyLinked<Element>>>>,
    tail : Option<Arc<RwLock<DoublyLinked<Element>>>>,
    val: Element
}


fn singleton<'x, Element>(val: Element) -> DoublyLinked<Element> {
  DoublyLinked { 
        head: None,
        tail: None,
        val: val
  }
}
fn add<Element>(tail: DoublyLinked<Element>, head: DoublyLinked<Element>) -> DoublyLinked<Element>  where Element: Clone {

    let tailRw = Arc::new(RwLock::new(tail));
    let headRw = Arc::new(RwLock::new(head));

    {
        let mut tailW = tailRw.write().unwrap();
        tailW.head = Some(headRw.clone())
    }
    {
        let mut headW = headRw.write().unwrap();
        headW.tail = Some(tailRw.clone())
    }

    let x = tailRw.read().unwrap().clone(); x
}

fn main() {
    let list = add(singleton(5), singleton(6));

    println!("hello 5 - {}", list.val);
    println!("hello 6 -  {}", list.head.clone().unwrap().read().unwrap().val);
    println!("hello 5 -  {}", list.head.clone().unwrap().read().unwrap().tail.clone().unwrap().read().unwrap().val);



    let list2 = add(list.head.clone().unwrap().read().unwrap().clone(), singleton(9));
    println!("hello 9 -  {}", list.head.clone().unwrap().read().unwrap().head.clone().unwrap().read().unwrap().val);

}
