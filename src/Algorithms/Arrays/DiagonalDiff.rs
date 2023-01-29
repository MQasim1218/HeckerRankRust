/**
.
*/
pub fn diagonal_difference(arr: &Vec<Vec<i32>>) -> i32 {
    let (mut tot_x, mut tot_y) = (0, 0);
    for i in 0..arr.len() {
        tot_x += arr[i][i];
        tot_y += arr[i][arr.len() - 1];
    }

    if tot_x > tot_y {
        tot_x - tot_y
    } else {
        tot_y - tot_x
    }
}
