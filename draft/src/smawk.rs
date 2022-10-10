/// SMAWK algorithm。
///
/// $A\_{i, j} = f(i, j)$ ($(i, j) \\in \\halfco{0}{n}^2$) で定まる
/// $n \\times n$ 行列 $A$ を考える。ただし、$A$ は totally monotone とする。
///
/// 各 $i\\in\\halfco{0}{n}$ について、$\\argmin\_{j\\in\\halfco{0}{n}} f(i, j)$
/// および $\\min\_{j\\in\\halfco{0}{n}} f(i, j)$ を求める。
pub fn smawk<T>(
    n: usize,
    f: impl FnMut(usize, usize) -> T,
) -> (Vec<usize>, Vec<T>) {
    todo!();
}
