pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2); // String型はコピートレートがないので、s2にs1の値がムーブされる

    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("Stack address of i1 is: {:?}", i1);
    println!("Stack address of i2 is: {:}", i2);
    println!("Stack address of i1 is: {:p}", &i1);
    println!("Stack address of i2 is: {:p}", &i2);

    // 文字列リテラル
    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    // sl2はsl1のコピーなのでsl1と同じアドレスを指しているが、ポインタのアドレスは異なる
    println!("Stack address of sl1 is: {:p}", sl1);
    println!("Stack address of sl2 is: {:p}", sl2);
    println!("Stack address of sl1 is: {:p}", &sl1);
    println!("Stack address of sl2 is: {:p}", &sl2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("Stack address of s3 is: {:p}", &s3);
    println!("Stack address of s4 is: {:p}", &s4);
    println!("Heap memory address of hello: {:?}", s3.as_ptr());
    println!("Heap memory address of hello: {:?}", s4.as_ptr());
    println!("{} {}", s3, s4);

    let s5 = String::from("hello");
    println!("Stack address of s5: {:p}", &s5);
    println!("Heap address of hello: {:?}", s5.as_ptr());
    println!("Len is : {}", s5.len());
    println!("Cap is : {}", s5.capacity());
    take_ownership(s5);
    // println!("{}", s5); // s5は所有権が関数にムーブされたので、ここでは使用できない
    let s6 = String::from("hello");
    println!("Stack address of s6: {:p}", &s6);
    println!("Heap address of hello: {:?}", s6.as_ptr());
    println!("Len of s6: {}", s6.len());
    println!("---------------------------------------");
    let s7 = take_giveback(s6);
    println!("Stack address of s7: {:p}", &s7);
    println!("Heap address of hello: {:?}", s7.as_ptr());
    println!("Len of s7: {}", s7.len());
    println!("---------------------------------------");
    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("The length of '{}' is {}.", s8, len);
    println!("---------------------------------------");
    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);
    println!("---------------------------------------");
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);
    println!("---------------------------------------");
    // let mut s10 = String::from("hello");
    // let r1 = &s10;
    // let r2 = &mut s10;
    //     "command": "workbench.action.terminal.resizePaneUp",
    //     println!("{} {} {}", s10, r1, r2);
    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    println!("{}", r1);
    println!("{}", s11);
    println!("---------------------------------------");
    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} and {}", r1, r2);
    let r3 = &mut s12;
    *r3 = String::from("hello_world");
    println!("{}", r3);
}

fn take_ownership(s: String) {
    println!("Stack address of s: {:p}", &s);
    println!("Heap address of s: {:?}", s.as_ptr());
    println!("Len of s: {}", s.len());
    println!("Cap is {}", s.capacity());
    println!("{}", s);
}
fn take_giveback(s: String) -> String {
    s
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(s: &mut String) {
    s.push_str("_world");
}
