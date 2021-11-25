use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ndarray::{Array2, arr2};
use simtro::routes;



pub fn make_terminus() -> Vec<(usize, usize)> {
    vec![(0, 19), (4, 8)]
}


fn make_adj_matrix_from_arcs(arcs: &[(usize, usize)]) -> Array2<u32> {
    let max = arcs.iter().copied().map(|(a, b)| if a > b {a} else {b}).max().unwrap() + 1;
    let mut output = Array2::zeros((max, max));

    for i in 0..max {
        for j in 0..max {
            output[(i, j)] = if i == j {
                0
            } else {
                u32::MAX
            };
        }
    }

    

    for (s, d) in arcs {
        output[(*s, *d)] = 1;
        output[(*d, *s)] = 1;
    }

    output
}

fn make_cross_arcs_list(count: usize) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut output = Vec::with_capacity(4 * count + 1);
    let mut north = 0;
    let mut south = 0;
    let mut east = 0;
    let mut west = 0;
    let mut i = 1;
    for _ in 0..count {

        let tmp = (north, i);
        north = i;
        i += 1;
        output.push(tmp);

        let tmp = (east, i);
        east = i;
        i += 1;
        output.push(tmp);

        let tmp = (south, i);
        south = i;
        i += 1;
        output.push(tmp);

        let tmp = (west, i);
        west = i;
        i += 1;
        output.push(tmp);

    }

    (output, vec![(north, south), (west, east)])
}


fn make_grid_arcs_list(count: usize, span: usize) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut arcs = Vec::with_capacity(4 * count + 1);
    let mut terms = Vec::with_capacity(4 * count + 1);
    
    let mut curr_hor = 0;
    let mut next = 1;
    for _ in 0..count {
        let arc = (curr_hor, next);
        arcs.push(arc);
        curr_hor = next;
        next += 1;
        let mut base = curr_hor;
        for _ in 0..span {
            let arc = (base, next);
            base += 1;
            next += 1;
            arcs.push(arc);
        }
        let t1 = next;
        let mut base = curr_hor;
        for _ in 0..span {
            let arc = (base, next);
            base += 1;
            next += 1;
            arcs.push(arc);
        }
        terms.push((t1, base));
    }
    terms.push((0, curr_hor));

    (arcs, terms)
}

fn criterion_benchmark(c: &mut Criterion) {
    let (arcs, term) = make_cross_arcs_list(300);
    c.bench_function("IPM-cross", |b| b.iter(|| routes::build_directions(make_adj_matrix_from_arcs(&arcs), &term)));

    let (arcs, term) = make_grid_arcs_list(300, 1);
    c.bench_function("IPM-grid", |b| b.iter(|| routes::build_directions(make_adj_matrix_from_arcs(&arcs), &term)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

