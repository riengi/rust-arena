// basic matrics operations with ndarray::arr2
use ndarray::arr2;

pub fn math_operations() {
    let m1 = arr2(&[[7, 8], [9, 10], [11, 12]]);
    let m2 = arr2(&[[2, 2], [2, 2], [2, 2]]);
    let m3 = arr2(&[[2, 2, 2], [2, 2, 2]]);

    // sum
    let m1_plus_m2 = &m1 + &m2;

    println!("{} + ", m1);
    println!("");
    println!("{} = ", m2);
    println!("");
    println!("{}", m1_plus_m2);

    // multiply
    let m1_multiply_m3 = m1.dot(&m3);
    println!("{}", m1_multiply_m3);
}
