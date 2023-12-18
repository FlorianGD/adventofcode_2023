use num::Complex;

//https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn val(&self) -> Complex<isize> {
        match self {
            Direction::Left => Complex::new(-1, 0),
            Direction::Right => Complex::new(1, 0),
            // imaginary axis is flipped
            Direction::Up => Complex::new(0, -1),
            Direction::Down => Complex::new(0, 1),
        }
    }
}
