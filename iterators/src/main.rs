//13.02 - Iterators are basically way to implement for loops, it performs similarly to for and is a zero cost abstraction
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn iterators_creating_new_iterators(){
    let v1: Vec<i32> = vec![1, 2, 3];
    //this creates a new iterator, has to be collected to be consumed.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2,3,4]);
}

//Can call next method in iterator manually
#[test]
fn iterator_demonstration(){
    let v1 = vec![1, 2, 3];

    // has to be defined as mutable to borrow
    let mut v1_iter = v1.iter();

    //Next is a consuming adapter, each time it's called, it's iterator is removed.
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(),None);
}

#[test]
fn iterator_sum(){
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    //v1_iter can't be used after summing as it has taken ownership.
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}