struct Body {
    m: f64,
    px: f64,
    py: f64,
    vx: f64,
    vy: f64,
}

impl Body {
    fn new(m: f64, px: f64, py: f64, vx: f64, vy: f64) -> Self {
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
