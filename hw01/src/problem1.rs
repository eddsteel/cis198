// problem1.rs

/// Computes the sum of all elements in the input i32 slice named `slice`
///

pub fn sum(slice: &[i32]) -> i32 {
    sum_func(slice)
}

// fn sum_imp(slice: &[i32]) -> i32 {
//     let mut total: i32  = 0;
//     for x in slice {
//         total += *x;
//     }
//     total
// }

fn sum_func(slice: &[i32]) -> i32 {
    slice.iter()
         .fold(0, |acc, &next| acc + next)
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let empty: Vec<i32> = vec![];
    vs.iter()
      .fold(empty, |mut acc, &next| {
                    if acc.contains(&next) { acc }
                    else {
                        acc.push(next);
                        acc
                    }})
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let empty: Vec<i32> = vec![];
    vs.iter()
        .fold(empty, |mut acc, &next| {
                      if pred(next) {
                          acc.push(next);
                          acc
                      }
                      else {
                          acc
                      }})
}
