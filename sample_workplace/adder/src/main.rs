use add_one;
use rand::Rng;
fn main() {
        let mut rng = rand::thread_rng();

        let num: i32 = rng.gen_range(0,100);

    println!("Number is{}, added:{}", num, add_one::add_one(num));
}
