pub fn run() {
    // stack over flow stack には8㎎までしか入らない
    // 8MB <= 出ないとエラーになる
    // let _a1: [u8; 9000000] = [1; 9000000];

    let mut v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    // Chech Adress
    println!("Stack address of v1 is: {:p}", &v1);
    println!("Stack address of v2 is: {:p}", &v2);
    println!("Heap memory address of v1: {:?}", v1.as_ptr());
    println!("Len of v1 is: {}", v1.len());
    println!("Capacity of v1 is: {}", v1.capacity());
    v1.insert(1, 10);
    println!("{:?}", v1);
    v1.remove(1);
    println!("{:?}", v1);
    // v1にv3を追加する
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);
}