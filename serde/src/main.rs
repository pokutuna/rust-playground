use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Line {
    #[serde(rename = "from")]
    a: Point,
    #[serde(rename = "to")]
    b: Point,
}

fn main() {
    let point = Point { x: 1, y: 2 };
    let serialized = serde_json::to_string(&point).unwrap();
    println!("{:?}", serialized);
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("{:?}", deserialized);

    let line = Line {
        a: Point { x: 0, y: 1 },
        b: Point { x: 3, y: 7 },
    };
    let serialized = serde_json::to_string(&line).unwrap();
    println!("{:?}", serialized);
    let deserialized: Line = serde_json::from_str(&serialized).unwrap();
    println!("{:?}", deserialized);
}
