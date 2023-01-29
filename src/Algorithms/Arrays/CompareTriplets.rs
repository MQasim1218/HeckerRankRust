fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut scores = vec![0, 0];

    for i in 0..3 {
        if a[i] > b[i] {
            scores[0] += 1;
        } else if a[i] < b[i] {
            scores[1] += 1;
        }
    }

    scores
}
