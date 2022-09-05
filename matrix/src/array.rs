// basic 1d, 2d, 3d arrays examples
pub fn compose_array() {
    println!("3d array 1x2x4");

    let mut arr1 = [1, 2, 3, 4];
    let arr2 = [1, 2, 3, 4];
    let arr3 = [1, 2, 3, 4];

    let arr2d1 = [arr1, arr2, arr3];
    let arr2d2 = [arr1, arr2, arr3];

    // Note int values are coppied
    arr1[0] = 10;

    let arr3d = [arr2d1, arr2d2];

    println!("{:?}", arr3d)
}

pub fn define_arrays() {
    // 1D array
    // [init_value; size]
    let arr = [0.1; 10];

    println!("1d array");
    for i in 0..arr.len() {
        println!("arr[{}] = {}", i, arr[i]);
    }

    // Matrix size is described like rows x columns
    // 2D array (6x4 matrix; aka 6 rows x 4 column)
    // To access Matrix value M2,3 = arr2d[1][2]
    let arr2d = [[0.1f64; 4]; 6];

    println!("2d array");
    for e in arr2d {
        print!("|  ");
        for f in e {
            print!("{}  ", f);
        }
        println!("|");
    }

    // 3D array, Matrix 2 x 5 x 10
    let arr3d = [[[0; 10]; 5]; 2];
    println!("Last element of 3d array = {:?}", arr3d[1][4][9]);
}
