use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, Debug)]
struct X(i32, i32, Option<i32>);


fn main() {
    println!("Hello, world!");

    // json 序列号tuple的时候，如果个数不对也会报错，需要显示声明undefined
    let x = serde_json::from_str::<X>("[1,1]");

    println!("print {:?}", x);
}
