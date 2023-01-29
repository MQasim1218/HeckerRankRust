mod CompareTriplets;
pub mod DiagonalDiff;
mod LongSum;
mod SimpleArraySum;

pub fn runner() {
    // ! Diagonal Difference
    // let diagonal_arr = vec![vec![1, 2, 3], vec![4, 5, 6], vec![9, 8, 9]];
    // DiagonalDiff::diagonal_difference(&diagonal_arr);

    // ! Simple Array Sum
    // let x = SimpleArraySum::simple_array_sum(vec![1, 2, 3, 4, 10, 11]);
    // println!("The Sum of all the elements in the array is: {}", x)

    // ! Long Array Sum
    let x = LongSum::long_array_sum(&[1, 2, 3, 4, 10, 11, 15, 17]);
    println!("The Sum of all the elements in the array is: {}", x)
}
