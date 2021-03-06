#![allow(dead_code)]
// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run(){
    let numbers: [i32; 4] = [1, 2, 3, 4];

    println!("{:?}", numbers);
    println!("single value: {}", numbers[0]);

    // mutable array
    let mut numbers_mut: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers_mut);

    // re-assign value
    numbers_mut[2] = 20;

    println!("{:?}", numbers_mut);

    // get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Arrays occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("slice: {:?}", slice);

    // pluralsight code
    let b = [1; 10] ;// 10 1s in the array
    let _c = [1u16; 10] ;// 16 bit ones are sent (takes less space in memory) 

    for i in 0..b.len(){
        println!("{}", b[i])
    }

    // multi dimentional array

    let matrix: [[f32;3]; 2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", matrix);

    for i in 0..matrix.len(){
        for j in 0..matrix[i].len(){
            if i == j{
                println!("matrix[{}][{}] = {}", i, j, matrix[i][j]);
            }
        }
    }
}