use rand::Rng;

const N: usize = 10000;
const MAXT: usize = 180000;

fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn max(numbers: &[i32]) -> i32 {
    *numbers.iter().max().unwrap()
}

fn main() {
    let mut rnd = rand::thread_rng();
    let mut cnt: [i32; N] = [0; N];
    for i in 0..N {
        let mut lc = 0;
        for t in 0..MAXT {
            let x: f32 = rnd.gen();
            if x <= 0.5 {
                lc -= 1;
            } else {
                lc += 1;
            }
            if lc == 0 {
                cnt[i] = t as i32;
                break;
            }
        }
    }
    println!("{}, {}", average(&cnt), max(&cnt));
}
