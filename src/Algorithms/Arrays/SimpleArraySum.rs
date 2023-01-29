/// Takes an array as an input and returns the sum of all the elements
pub fn simple_array_sum(arr: Vec<i32>) -> i32 {
    let mut tot = 0;
    for ind in 0..arr.len() {
        tot += arr[ind];
    }
    tot
}
