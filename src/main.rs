struct Buffer<T> {
    buf: Vec<T>,
}

impl<T> Buffer<T> {
    pub fn sum(&self) -> T {
        self.buf.iter().sum()
    }
}

fn main() {
    println!("Hello, world!");
}
