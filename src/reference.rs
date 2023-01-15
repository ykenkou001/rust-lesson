pub fn run() {
    let s1 = String::from("hello");
    let sr1 = &s1;
    let sr2 = &s1;
    println!("Stack address of s1: {:p}", &s1);
    println!("Heap address of hello: {:?}", s1.as_ptr());
    println!("Len is : {}", s1.len());
    println!("Cap is : {}", s1.capacity());
    println!("Value of reference sr1: {:p}", sr1);
    println!("Value of reference sr2: {:p}", sr2);
    println!("Stack address of sr1: {:p}", &sr1);
    println!("Stack address of sr2: {:p}", &sr2);
}