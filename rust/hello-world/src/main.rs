fn main() {
    println!("Hello, world!");
    println!("{}", test_func(Json(32)));
}

pub struct Json<T>(pub T);

fn test_func(Json(val): Json<u32>) -> u32 {
    val
}