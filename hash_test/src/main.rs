use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

#[derive(Debug, )]
enum Op {
    Clean(i32),
    Render(u32),
    Work(String),
}

impl Hash for Op {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // match self {
        //     Op::Clean(_) => state.write_u8(0),
        //     Op::Render(_) => state.write_u8(1),
        //     Op::Work(_) => state.write_u8(2),
        // };

        // state.finish();
    }
}


impl PartialEq for Op {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Op::Clean(_), Op::Clean(_)) => true,
            (Op::Render(_), Op::Render(_)) => true,
            (Op::Work(_), Op::Work(_)) => true,
            _ => false
        }
    }
}
impl Eq for Op {}


fn main() {
    let a: u32= 3;
    let b : u32= 2;
    let c : u32= 4;
    let d : u32= 3;

    assert!((a - b) == (c - d));

    println!("Hello, world!");
    let op_render1 = Op::Render(333);
    let op_render2 = Op::Render(22);

    let op_clean1 = Op::Clean(444);
    let op_clean2 = Op::Clean(555);

    let op_work1 = Op::Work("work1".to_string());
    let op_work2 = Op::Work("work2".to_string());

    assert!(op_render1 == op_render2);
    assert!(op_clean1 == op_clean2);
    assert!(op_clean2 != op_work2);

    let mut set = std::collections::HashSet::new();

    set.replace(op_render1); // 333
    // set.remove(&op_render2); // 333
    set.replace(op_render2); // 22
    set.replace(op_clean1); // 444
    // set.remove(&op_clean2); // 555
    set.replace(op_clean2); // 555
    set.replace(op_work1); // work1
    // set.remove(&op_work2); // work2
    set.replace(op_work2); // work2

    set.iter().for_each(|sss| {
        println!("set op: {:?}", sss);
    })
}
