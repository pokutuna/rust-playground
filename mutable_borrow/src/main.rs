struct Value {
    v: i32,
}

fn main() {
    let mut value = Box::new(Value { v: 1 });

    let borrowed1 = &value;
    let borrowed2 = &value;

    println!("{}, {}, {}", value.v, borrowed1.v, borrowed2.v);

    let mutable_borrow = &mut value;

    // println!("{}", borrowed1);
    // ↑ これがあるときは mutable_borrow できない
    // v の借用があるときにミュータブルな借用はできない、下に借用が1つもない場合はできる

    mutable_borrow.v = 2;

    println!("{}", value.v);
}
