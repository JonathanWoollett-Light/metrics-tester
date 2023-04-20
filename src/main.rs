use rand::Rng;
use std::io::Write;

#[derive(serde::Serialize)]
struct MyMetrics {
    signed_integer: i32,
    unsigned_integer: u32,
    float: f32,
}
impl rand::distributions::Distribution<MyMetrics> for rand::distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MyMetrics {
        MyMetrics {
            signed_integer: rng.gen(),
            unsigned_integer: rng.gen(),
            float: rng.gen(),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("metrics.json")
        .unwrap();
    let value = rand::thread_rng().gen::<MyMetrics>();
    let json_str = serde_json::to_string(&value).unwrap();
    file.write_all(json_str.as_bytes()).unwrap();
}
