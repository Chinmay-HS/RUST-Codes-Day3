// 1. SCOPE AND AUTOMATIC MEMORY FREEDOM (RAII)

fn main() {
    let owner = String::from("I own this string");
    println!("{}", owner);
} // owner goes out of scope here — Rust automatically frees it

// 2. MOVE SEMANTICS (OWNERSHIP TRANSFER)
fn main() {
    let original = String::from("hello world");
    let moved = original;
    println!("moved: {}", moved); // only use the new owner
}

// 3. CLONING (DEEP HEAP COPY)

fn main() {
    let original = String::from("clone me");
    let cloned = original.clone(); // deep copy, new heap memory
    println!("original: {}", original);
    println!("cloned: {}", cloned); // both valid
}

// 4. THE COPY TRAIT (STACK TYPES)
//
fn main() {
    let x: i32 = 10;
    let y = x; // copy, not a move — i32 implements Copy
    println!("x: {}, y: {}", x, y); // both valid
}
