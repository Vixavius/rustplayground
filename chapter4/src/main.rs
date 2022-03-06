fn main() {
    heap_objects_ownership();
    stack_objects_ownership();
    function_ownership();
    borrowed_references();
    mutable_references();
}

fn heap_objects_ownership() {
    // Heap objects ownership
    let s1 = String::from("hello"); // Heap-allocated string
    let s2 = s1; // Not a copy! s1 is moved into s2

    // println!("{}, world!", s1); -- compile error: "value is borrowed here after move"
    println!("{}, world!", s2);

    let s3 = s2.clone(); // Deep copy
    println!("{}, s2 world!", s2); // No move occurs for s2
    println!("{}, s3 world!", s3);
}

fn stack_objects_ownership() {
    // Stack objects ownership
    let x = 5;
    let y = x; // Copy, no move occurs

    println!("{}, {}", x, y);
}

// --------------------
// Function Ownership
// --------------------

fn function_ownership() {
    let s = String::from("Hello");

    takes_ownership(s); // s moves into the function

    //println!("{}", s); // compile error, s has been cleared

    let x = 5;
    makes_copy(x);
    println!("{}", x); // Value was copied because i32 is Copy

    let s1 = gives_ownership();

    println!("{}, world", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("{}, world", s3);

    // Referenceless approach to doing something useful with ownership transfer
    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);

    println!("The length of: {} is: {}", s5, len);
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();

    (some_string, length)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Drop is called here, freeing the memory for the String

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string // String is returned
}

fn takes_and_gives_back(some_string: String) -> String {
    // Comes into scope, then is returned
    some_string
}

// --------------------
// Reference Borrowing
// --------------------

fn borrowed_references() {
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("The length of '{}' is {}", s1, len);

    let s2 = String::from("hello");
    change(&s2);
}

fn calculate_length_ref(some_string: &String) -> usize {
    // some_string is functionally a ptr to s1 which is itself a ptr to
    // 'hello' on the heap
    some_string.len()
} // s1 is not cleared here because this function doesn't own it, it is _borrowed_

fn change(_some_string: &String) {
    // some_string.push_str(", world"); // compile-error: some_string is borrowed here, so it cannot be mutated
}

// --------------------
// Mutable References
// --------------------

fn mutable_references() {
    let mut s = String::from("hello");

    change_mut_ref(&mut s); // Ownership passes to the function

    println!("{}", s);

    s.push_str(", I'm here!"); // The function returns the ownership of sS

    println!("{}", s);

    let r1 = &mut s; // r1 now is a mutable reference to s
                     // let r2 = &mut s; // duplicate mut ref, cannot use

    r1.push_str(" How does it go?");

    // println!("{}", s); // s can't be used here
    println!("{}", r1);

    // s can now be used again
    s.push_str(" nah");
    println!("{}", s);

    // But, r1 cannot
    // r1.push_str(" what");
    // println!("{}", r1);

    // New Scope
    {
        let _r1 = &mut s;
    } // s can again be referenced as r1 has gone out of scope

    let mut s = String::from("Hello");

    // Can have any number of immutable references
    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // We can't borrow s as mutable because it's already borrowed as mutable.
    // let r3 = &mut s;

    println!("{} {} {}", r1, r2, s);
    s.push_str(", world");
}

fn change_mut_ref(some_string: &mut String) {
    some_string.push_str(", world");
}
