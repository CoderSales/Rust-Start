// use derive keyword to generate implementations of Copy and Clone
#[derive(Copy, Clone)]
struct MyStruct {
    value: i32,
}

fn main() {
    let x = MyStruct { value: 10 };
    let y = x;

    println!("x: {:?}", x.value);
    println!("y: {:?}", y.value);
}