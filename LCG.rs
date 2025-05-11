struct LCG {
    state: u64,
    a: u64,
    c: u64,
    m: u64,
}

impl LCG {
    // Инициализация генератора с seed
    fn new(seed: u64) -> Self {
        LCG {
            state: seed,
            a: 16807,
            c: 0,
            m: 2147483647, // 2^31 - 1
        }
    }

   
    fn next(&mut self) -> f64 {
        self.state = (self.a * self.state + self.c) % self.m;
        self.state as f64 / self.m as f64
    }
}

fn main() {
    let mut lcg = LCG::new(12345); // Seed = 12345

    // Генерация 10 псевдослучайных чисел
    for _ in 0..10 {
        println!("{}", lcg.next());
    }
}
