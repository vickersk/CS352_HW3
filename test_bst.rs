mod bst;

fn main() {
    // build sample tree containing 3, 5
    let mut sample = bst::Bst::new(5);
    sample.insert(3);
    // clone
    let sample2 = sample.clone();
    // add 7 to original tree (should not affect clone)
    sample.insert(7);
    
    // print trees
    println!("{}\n{}", sample, sample2);
    for val in sample.iter() {
        println!("{}", val);
    }
}