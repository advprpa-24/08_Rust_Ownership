// Look at the different ways assignments work
// depending on Copy and Clone traits.

#[derive(Debug)]
struct A {
    i: i32
}

fn no_clone_no_copy() {
    let a = A { i : 5};
    // let _other = a; // Uncomment and compile
    
    println!("{:?}", a);
}

#[derive(Debug, Clone)]
struct B {
    i: i32
}

fn only_clone() {
    let b = B { i : 5};
    // let _other = b; // Uncomment and compile
    
    println!("{:?}", b);
}

#[derive(Debug, Clone, Copy)]
struct C {
    i: i32
}

fn clone_and_copy() {
    let c = C { i : 5};
    // let _other = c; // Uncomment and compile
    
    println!("{:?}", c);
}


fn main() {
    no_clone_no_copy();
    only_clone();
    clone_and_copy();
}