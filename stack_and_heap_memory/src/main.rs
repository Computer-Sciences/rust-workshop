fn main() {
    println!("=== STACK ====\n");
    println!("values stored in sequential order of insertion");
    println!("data added in LIFO (last in first out)");
    println!("stores variables - pushing values on the stack");
    println!("also holds info for function execution");
    println!(
        "stack have very fast access because no guessing where to put data, it will be on top"
    );
    println!("stacks are limited in size");
    println!("all data in stack must have a known fixed size\n");
    func1();
    println!("func1 done");
    println!("pop variable y off the stack");
    println!("pop variable z off the stack\n");

    println!("\n\n=== HEAP ====\n");
    println!("adding data to heap, search for large enough place in memory to store");
    println!("marks memory spot as being used (allocating) and put data in it");
    println!("accessing data in heap is more complex than the stack because stack allocates anywhere in available memory");
    println!("slower than stack");
    println!("dynamically add and remove data");
    println!("\n\n=== POINTER ====\n");
    println!("data type that store a memory address");
    println!("pointers have a fixed size so can be stored on the stack");
    println!("adding and accessing data on the heap is done through pointers");
}

fn func1() {
    println!("func1 executing...");
    let y = 3.11;
    println!("push variable y = {} onto the stack", y);
    let z = 5;
    println!("push variable z = {} onto the stack", z);
    func2();
    println!("func2 done");
    println!("pop variable arr off the stack");
}

fn func2() {
    println!("func2 executing...");
    let arr = [2, 3, 4];
    println!("push variable arr = {:?} onto the stack", arr);
}
