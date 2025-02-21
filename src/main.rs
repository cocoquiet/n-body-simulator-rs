struct Body {
    m: i64,
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl Body {
    fn new(m: i64, px: i64, py: i64, vx: i64, vy: i64) -> Self {
        Body {
            m: m,
            px: px,
            py: py,
            vx: vx,
            vy: vy,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
