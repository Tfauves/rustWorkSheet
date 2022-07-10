fn main() {
    let n = Numbers { a: 3, b: 2 };
    n.adds(n.a, n.b);
    n.subtract(n.a, n.b)
}

struct Numbers {
    a: i32,
    b: i32,
}

impl Numbers {
    fn adds(&self, n1: i32, n2: i32) {
        println!("{} + {} = {}", n1, n2, n1 + n2);
    }

    fn subtract(&self, n1: i32, n2: i32) {
        println!("{} - {} = {}", n1, n2, n1 - n2);
    }
}
