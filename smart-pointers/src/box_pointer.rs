// data structures on heap
use std::ops::Deref;

// Box - pointer type for heap allocation
// std::ops::Deref::deref() derefences Box<type> to &type
// Box::new() allocates new data on heap
pub fn int_box() {
    // allocate
    let mut box_pointer = Box::new(5);
    println!("pointer = {}", box_pointer);

    // modify value inside box
    *box_pointer += 1;
    println!("pointer = {}", *box_pointer);

    // get value into new variable
    let value = box_pointer.deref();
    println!("value = {}", value);

    // Clone pointer with its value
    let mut box_pointer_clone = box_pointer.clone();
    *box_pointer_clone = *box_pointer_clone + 1;
    println!("clone pointer = {}", *box_pointer_clone);
    println!("pointer = {}", *box_pointer);

    // New pointer (moving ownership or reference)
    let mut new_box_pointer = box_pointer;
    println!("new pointer = {}", *new_box_pointer);

    // Two referenced pointers (same data)
    {
        let ref1 = &new_box_pointer;
        let ref2 = &new_box_pointer;
        println!("ref1 = {}", *ref1);
        println!("ref2 = {}", *ref2);
    }

    // One mutable pointer reference
    let mut_ref = &mut new_box_pointer;
    println!("mut_ref = {}", *mut_ref);
}
