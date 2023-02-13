use std::ops::AddAssign;

/**
 *
*/

/// Calculate the diagonal different of an array
/// Emaple below
///                 0   2   4
///                 2   3   5
///                 7   4   2
///
pub(crate) fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let (mut tot_x, mut tot_y) = (0, 0);

    for i in 0..arr.len() {
        tot_x += arr[i][i];
        tot_y += arr[arr.len() - i - 1][i];
    }
    // if tot_x > tot_y {
    let res = (tot_x - tot_y).abs();
    println!("{}", res);

    res
    // } else {
    // tot_y - tot_x
    // }
}
