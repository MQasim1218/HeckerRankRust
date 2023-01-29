/// Takes an array as an input and returns the sum of all the elements
pub fn long_array_sum(arr: &[i64]) -> i64 {
    println!("{:?}", arr);
    let mut tot: i64 = 0;
    for val in arr {
        tot += val;
    }
    tot
}
