struct Player {
    name: String,
    goal_count: u32,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn structs() {
    println!("\n\n--- Structs ---\n");
    basic_syntax();
    debugging();
    method_syntax();
}

fn basic_syntax() {
    let messi = Player {
        name: String::from("Messi"),
        goal_count: 826,
    };
    println!("{}'s total goal: {}", messi.name, messi.goal_count);

    let mut messi = Player {
        name: String::from("Messi"),
        goal_count: 826,
    };
    messi.goal_count += 1;

    let _ronaldo = build_player(String::from("Ronaldo"), 885);

    let _messi_jr = Player {
        goal_count: 0,
        ..messi
    };
    // println!("{}", messi.name); // messi.name ownership is moved
    println!("goal count: {}", messi.goal_count); // but messi.goal_count is copied
}

fn build_player(name: String, goal_count: u32) -> Player {
    Player { name, goal_count }
}

fn debugging() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("and the area is {}", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    println!("rect1 is {:?}", rectangle);
    rectangle.width * rectangle.height
}

fn method_syntax() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {}", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    non_method_associated_functions();
}

fn non_method_associated_functions() {
    let sq = Rectangle::square(3);
    println!("The area of the squqre is {}", sq.area());
}
