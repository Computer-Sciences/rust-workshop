use std::ops::{Add, MulAssign};

fn sum_boxes<T: Add<Output = T> + MulAssign + Copy>(mut b1: Box<T>, b2: Box<T>) -> Box<T> {
    *b1 *= *b2;
    Box::new(*b1 + *b2)
}

fn main() {
    let box1 = Box::new(11);
    let box2 = Box::new(44);
    assert_eq!(*sum_boxes(box1, box2), 528);

    let box1 = Box::new(1.1);
    let box2 = Box::new(4.4);
    assert_eq!(*sum_boxes(box1, box2), 9.240000000000002);

    let box1 = Box::new(1.1f32);
    let box2 = Box::new(4.4f32);
    assert_eq!(*sum_boxes(box1, box2), 9.24f32);

    let box1 = Box::new(11u16);
    let box2 = Box::new(44u16);
    assert_eq!(*sum_boxes(box1, box2), 528u16);

    println!("tests passed!");
}
