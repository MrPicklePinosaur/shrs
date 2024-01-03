//! This script simulates multi-dimensional matrix calculations.

const SIZE = 50;

fn new_mat(x, y) {
    let row = [];
    row.pad(y, 0.0);

    let matrix = [];
    matrix.pad(x, row);

    matrix
}

fn mat_gen() {
    const n = global::SIZE;
    const tmp = 1.0 / n / n;
    let m = new_mat(n, n);

    for i in 0..n {
        for j in 0..n {
            m[i][j] = tmp * (i - j) * (i + j);
        }
    }

    m
}

fn mat_mul(a, b) {
    let b2 = new_mat(a[0].len, b[0].len);

    for i in 0..a[0].len {
        for j in 0..b[0].len {
            b2[j][i] = b[i][j];
        }
    }

    let c = new_mat(a.len, b[0].len);

    for i in 0..c.len {
        for j in 0..c[i].len {
            c[i][j] = 0.0;

            for z in 0..a[i].len {
                c[i][j] += a[i][z] * b2[j][z];
            }
        }
    }

    c
}

const now = timestamp();

const a = mat_gen();
const b = mat_gen();
const c = mat_mul(a, b);

/*
for i in 0..SIZE) {
    print(c[i]);
}
*/

print(`Finished. Run time = ${now.elapsed} seconds.`);
