use std::error::Error;

fn main() {
    let v: Vec<u8> = vec![1, 2, 3, 4, 5];

    if let &[one, two, three, ..] = &v[..] {
        println!("{:?}, {:?}, {:?} with if-let", one, two, three);
    }
    match &v[..] {
        &[one, two, three, ..] => println!("{:?}, {:?}, {:?} with match", one, two, three),
        _ => (),
    }

    let res: Result<Vec<u8>, Box<dyn Error>> = Ok(v);

    if let Ok(&[one, two, three, ..]) = res.as_deref() {
        println!("{:?}, {:?}, {:?} with Result#as_deref", one, two, three);
    }
}
