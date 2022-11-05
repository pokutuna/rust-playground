struct User {
    id: u8,
    name: String,
    age: u8,
}

impl User {
    pub fn greeting(&self) {
        println!("Hello! My name is {}, {} years old", self.name, self.age)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.age.cmp(&other.age))
    }
}

fn main() {
    let c = User {
        id: 1,
        name: "cocoa".into(),
        age: 15,
    };
    c.greeting();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare() {
        let cocoa = User {
            id: 1,
            name: "cocoa".into(),
            age: 15,
        };
        let cocoa_now = User {
            id: 1,
            name: "now cocoa".into(),
            age: 18,
        };
        let chino = User {
            id: 2,
            name: "chino".into(),
            age: 14,
        };

        assert!(cocoa == cocoa_now);
        assert!(cocoa != chino);
    }

    #[test]
    fn sort() {
        let cocoa = User {
            id: 1,
            name: "cocoa".into(),
            age: 15,
        };
        let cocoa_now = User {
            id: 1,
            name: "now cocoa".into(),
            age: 18,
        };
        let chino = User {
            id: 2,
            name: "chino".into(),
            age: 14,
        };
        let chino_1yago = User {
            id: 2,
            name: "chino".into(),
            age: 15,
        };

        assert!(cocoa < cocoa_now);
        assert!(cocoa <= cocoa_now);
        assert!(chino < cocoa);
        assert!(!(chino_1yago > cocoa));
    }
}
