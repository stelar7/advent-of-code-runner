use std::io::BufRead;

use ndarray::{Array1, Array2, ArrayView2, ArrayViewMut2, Axis, Zip};

pub fn main() {
    let mut h = 0;
    let grid = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .inspect(|_| h += 1)
        .flat_map(|line| line.into_bytes().into_iter().map(|b| (b - b'0') as i8))
        .collect::<Vec<_>>();

    let w = grid.len() / h;
    let h = h;

    let grid = Array2::from_shape_vec((h, w), grid).unwrap();

    let mut visible = Array2::from_elem(grid.raw_dim(), false);
    apply4::<Visible>(grid.view(), visible.view_mut());
    let p1 = visible.iter().copied().filter(|&visible| visible).count();

    let mut prominence = Array2::from_elem(grid.raw_dim(), 1_usize);
    apply4::<Prominence>(grid.view(), prominence.view_mut());
    let p2 = prominence.iter().copied().max().unwrap();

    println!("{}\n{}", p1, p2);
}

/// Apply sweep in all four directions.
fn apply4<S>(grid: ArrayView2<i8>, mut out: ArrayViewMut2<S::OutElem>)
where
    S: Sweep,
{
    apply2::<S>(grid, out.view_mut());
    apply2::<S>(grid.view().reversed_axes(), out.view_mut().reversed_axes());
}

/// Apply sweep in top-down and bottom-up directions.
fn apply2<S>(mut grid: ArrayView2<i8>, mut out: ArrayViewMut2<S::OutElem>)
where
    S: Sweep,
{
    S::apply(grid, out.view_mut());
    grid.invert_axis(Axis(0));
    out.invert_axis(Axis(0));
    S::apply(grid, out);
}

/// Sweep, applied only in top-down direction.
trait Sweep {
    type OutElem;

    fn apply(grid: ArrayView2<i8>, out: ArrayViewMut2<Self::OutElem>);
}

struct Visible;
impl Sweep for Visible {
    type OutElem = bool;

    #[inline(always)]
    fn apply(grid: ArrayView2<i8>, mut visible: ArrayViewMut2<Self::OutElem>) {
        let mut row_top = Array1::from_elem((grid.shape()[0],), 0_i8);
        Zip::from(grid.axis_iter(Axis(0)))
            .and(visible.axis_iter_mut(Axis(0)))
            .for_each(|row, row_vis| {
                Zip::from(&mut row_top)
                    .and(row_vis)
                    .and(row)
                    .for_each(|a_top, a_vis, &a| {
                        *a_vis |= *a_top < a + 1;
                        *a_top = std::cmp::max(*a_top, a + 1);
                    });
            });
    }
}

struct Prominence;
impl Sweep for Prominence {
    type OutElem = usize;

    #[inline(always)]
    fn apply(grid: ArrayView2<i8>, mut prominence: ArrayViewMut2<Self::OutElem>) {
        // For the given height, what row index was it last seen at.
        let mut height_locs = Array2::from_elem((grid.shape()[0], 10), 0_usize);
        Zip::indexed(grid.axis_iter(Axis(0)))
            .and(prominence.axis_iter_mut(Axis(0)))
            .for_each(|idx, row, row_prom| {
                Zip::from(height_locs.axis_iter_mut(Axis(0)))
                    .and(row_prom)
                    .and(row)
                    .for_each(|mut a_height_locs, a_prom, &a| {
                        let a = a as usize;
                        *a_prom *= idx - a_height_locs[a];
                        // Set all the height locs 0..=a to idx, cause `a` blocks those heights.
                        a_height_locs.slice_axis_mut(Axis(0), (0..=a).into()).fill(idx);
                    })
            })
    }
}
