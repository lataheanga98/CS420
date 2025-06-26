// Demonstrates variable shadowing and inheritance-like behavior

struct C1 {
    x: i32,
    y: i32,
}

impl C1 {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn set_x1(&mut self, v: i32) {
        self.x = v;
    }

    fn set_y1(&mut self, v: i32) {
        self.y = v;
    }

    fn get_x1(&self) -> i32 {
        self.x
    }

    fn get_y1(&self) -> i32 {
        self.y
    }
}

struct C2 {
    base: C1,
    y: i32, // shadows C1's y
}

impl C2 {
    fn new() -> Self {
        Self {
            base: C1::new(),
            y: 0,
        }
    }

    fn set_x1(&mut self, v: i32) {
        self.base.set_x1(v);
    }

    fn set_y1(&mut self, v: i32) {
        self.base.set_y1(v);
    }

    fn set_y2(&mut self, v: i32) {
        self.y = v;
    }

    fn get_x1(&self) -> i32 {
        self.base.get_x1()
    }

    fn get_y1(&self) -> i32 {
        self.base.get_y1()
    }

    fn get_x2(&self) -> i32 {
        self.base.x
    }

    fn get_y2(&self) -> i32 {
        self.y
    }
}

pub fn run() {
    println!("🔍 Running variable inheritance test...");

    let mut c2 = C2::new();
    c2.set_x1(10); // set inherited x
    c2.set_y1(20); // set base.y
    c2.set_y2(30); // set shadowed y

    println!("Base x (base.x): {}", c2.get_x1()); // expect 10
    println!("Base y (base.y): {}", c2.get_y1()); // expect 20
    println!("Shadowed y (C2.y): {}", c2.get_y2()); // expect 30
    println!("get_x2(): {}", c2.get_x2());

}
