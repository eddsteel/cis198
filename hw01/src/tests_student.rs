#![cfg(test)]

use problem1::{sum, dedup, filter};
use problem2::{mat_mult, Matrix};
use problem3::sieve;
use problem4::{hanoi, Peg};

#[test]
fn test_sum_empty() {
    let array = [];
    assert_eq!(sum(&array), 0);
}

#[test]
fn test_sum_single() {
    let array = [1];
    assert_eq!(sum(&array), 1);
}


#[test]
fn test_dedup_empty() {
    let vs = vec![];
    assert_eq!(dedup(&vs), vs);
}

#[test]
fn test_dedup_repeated() {
    let vs = vec![1,1,1,1,1];
    assert_eq!(dedup(&vs), vec![1]);
}

#[test]
fn test_dedup_rando() {
    let vs = vec![1,2,1,3,1,4,2,5,3,6];
    assert_eq!(dedup(&vs), vec![1,2,3,4,5,6]);
}

#[test]
fn test_filter_false_empty() {
    let vs = vec![];
    assert_eq!(filter(&vs, &(|x| false)), vec![])
}

#[test]
fn test_filter_true_singleton() {
    let vs = vec![1];
    assert_eq!(filter(&vs, &(|x| true)), vec![1])
}

#[test]
fn test_filter_true_some() {
    let vs = vec![1,2,3];
    assert_eq!(filter(&vs, &(|x| true)), vec![1,2,3]);
}

#[test]
fn test_filter_false_some() {
    let vs = vec![1,2,3];
    assert_eq!(filter(&vs, &(|x| false)), vec![]);
}

#[test]
fn test_filter_odds() {
    let vs = vec![1,2,3,4,5];
    fn odd(x: i32) -> bool {
        ! x & 1 == 0
    }
    assert_eq!(filter(&vs, &odd), vec![1,3,5]);
}

//#[test]
fn boom() {
    let m1: Vec<Vec<f32>> = vec![vec![1.0, 2.0]];
    let m2: Vec<Vec<f32>> = vec![vec![1.0],vec![2.0],vec![3.0]];

    assert_eq!(mat_mult(&m1, &m2), vec![[]]);
}

#[test]
fn mat_mult_single() {
    let res = mat_mult(&(vec![vec![1.0]]), &(vec![vec![1.0]]));

    assert_eq!(res, vec![vec![1.0]]);
}

#[test]
fn mat_mult_more_involved() {
    let mat1: Matrix = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
    let mat2: Matrix = vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]];
    let emat: Matrix = vec![vec![58.0, 64.0], vec![139.0, 154.0]];

    let res: Matrix = mat_mult(&mat1, &mat2);
    assert_eq!(res, emat);
}
