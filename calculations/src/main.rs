use std::thread;
use std::time::Duration;
use std::collections::HashMap;

//13.01 Closures
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    
    let x = 4;
    //can access x directly in closure, in scope variables are accessible
    let equal_to_x = |z| z == x;
    // to take ownership `let equal_to_x = move |z| z == x;`

    let y = 4;

    assert!(equal_to_x(y));
    }

fn generate_workout(intensity: u32, random_number: u32) {
    //type will get inferred on first
    let mut expensive_closure = Cacher::new(|num| {
        println!("Calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today do {} pushups", expensive_closure.value(intensity));
        println!("Next do {} situps",  expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes",  expensive_closure.value(intensity));
        }
    }
}

//Implement caching
struct Cacher<T> where T: Fn(u32) -> u32{
    calculation: T,
    value: HashMap<u32, u32>,
    }

impl<T> Cacher<T> 
    where T: Fn(u32) -> u32{
        fn new(calculation: T) -> Cacher<T>{
            Cacher{
                calculation,
                value: HashMap::new(),
                }
        }

        fn value(&mut self, arg: u32) -> u32{
           // execute calculation if value not populated
            match self.value.get(&arg){
                Some(v) => v.clone(),
                None => {
                    let result = (self.calculation)(arg);
                    let arg = arg.clone();
                    self.value.insert(arg, result);
                    result
                },
            }            
        }
    }


//Try implementation with hashmap to fix this.
#[test]
fn call_with_different_values(){
    let mut c = Cacher::new(|a| a);
    let v1  = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}