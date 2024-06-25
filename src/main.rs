fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }
//
// fn main() {
//     // 1번
//     const VALUE: u8 = 5;
//     println!("The value of x is: {VALUE}");
//
//     // 2번
//     const VALUE2: u8 = VALUE * 5;
//     println!("The value of x is: {VALUE2}");
//
//     // 3번
//     let x =  5;
//     const VALUE3: u8 = x * 5;
//     println!("The value of x is: {VALUE3}");
//
// }
//
// fn main() {
//     let x = 5; // 여기서 x의 값은 5
//
//     let x = x + 1; // 여기서 x의 값은 6
//
//     {
//         let x = x * 2; // 여기서 x의 값는 12
//         println!("The value of x in the inner scope is: {x}");
//     }
//
//     // 스코프를 벗어나면서 여기서 x의 값는 6으로 되돌아옴
//     println!("The value of x is: {x}");
// }