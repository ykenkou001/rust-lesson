// mod ownership;
// mod reference;
// mod stack_heap;
// mod vars;
// mod generics;
// mod lifetime;
// mod structs;
// mod enums;
// mod traits;
extern crate lib_demo;
mod error_handling;
mod unit_test;
fn main() {
    //     stack_heap::run();
    //     vars::run();
    // ownership::run();
    // reference::run();
    // generics::run();
    // lifetime::run();
    // structs::run();
    // enums::run();
    // traits::run();
    error_handling::run();
    // unit_test::run();
    //     vars::sub_a::func_a();
    //     vars::sub_b::func_b();
    lib_demo::print_random_number();
}
