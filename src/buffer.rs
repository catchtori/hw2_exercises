struct Buffer<T> {
    pub buf: Vec<T>,
}

impl<T> Buffer<T> {
    // create a buffer
    pub fn new() -> Self {
        Buffer { buf: Vec::new() }
    }

    // add elements in the buffer
    pub fn push(&mut self, item: T) {
        self.buf.push(item);
    }

    // calculate the sum
    pub fn sum(&self) -> T
    where
        T: std::ops::Add<Output = T> + Default + Clone,
    {
        self.buf.iter().cloned().fold(T::default(), |acc, x| acc + x)
    }
}

fn main() {
    let mut buffer = Buffer::new();
    buffer.push(1);
    buffer.push(2);
    buffer.push(3);

    let sum = buffer.sum();
    println!("Sum: {:?}", sum); // 输出 Sum: 6
}
