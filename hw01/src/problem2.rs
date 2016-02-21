/// Represents a matrix in row-major order
// [[1,2,3],[1,2,3]]
//
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let (r1, c1) = size(mat1);
    let (r2, c2) = size(mat2);

    if r1 != c2 { panic!("Can't dot multiply these mismatched matrices"); }

    let mut res: Vec<Vec<f32>> = Vec::new();

    (0..r1).fold(res, |mut macc, nextr| {
        let row = (0..c2).fold(Vec::new(), |mut acc, nextc| {
            // woof. When to build/copy, when to pass by ref?
            let val = mat_combine(extractrow(mat1, nextr), &extractcol(mat2, nextc));
            acc.push(val);
            acc
        });

        macc.push(row);
        macc
    })
}

fn mat_combine(r: &Vec<f32>, c: &Vec<f32>) -> f32 {
    (0..r.len()).fold(0.0, |acc, n| {
        acc + (r[n] * c[n])
    })
}

fn extractrow(mat: &Matrix, idx: usize) -> &Vec<f32> {
    &(mat[idx])
}


fn extractcol(mat: &Matrix, idx: usize) -> Vec<f32> {
    let mut col: Vec<f32> = Vec::new();
    for row in mat {
        col.push(row[idx]);
    }
    col
}

fn size(mat: &Matrix) -> (usize, usize) {
    (num_rows(mat), num_cols(mat))
}

fn num_rows(mat: &Matrix) -> usize {
    mat.len()
}

fn num_cols(mat: &Matrix) -> usize {
    if mat.len() > 0 {
        mat[0].len()
    } else {
        0
    }
}
