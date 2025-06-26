trait Base {
    fn m1(&self) -> i32;
    fn m2(&self) -> i32;
}

struct C1;

impl Base for C1 {
    fn m1(&self) -> i32 {
        self.m2()
    }

    fn m2(&self) -> i32 {
        13
    }
}

struct C2;

impl Base for C2 {
    fn m1(&self) -> i32 {
        22
    }

    fn m2(&self) -> i32 {
        23
    }
}

struct C3;

impl Base for C3 {
    fn m1(&self) -> i32 {
        32
    }

    fn m2(&self) -> i32 {
        33
    }
}

// Demonstrates dynamic dispatch using a trait object
fn dynamic_dispatch(obj: &dyn Base) -> i32 {
    obj.m1()
}

pub fn run() {
    println!("🔍 Running method dispatch test...");

    let c1 = C1;
    let c2 = C2;
    let c3 = C3;

    println!("C1 m1() = {}", dynamic_dispatch(&c1)); // calls C1.m1(), which calls C1.m2() => 13
    println!("C2 m1() = {}", dynamic_dispatch(&c2)); // calls C2.m1() => 22
    println!("C3 m1() = {}", dynamic_dispatch(&c3)); // calls C3.m1() => 32
}
