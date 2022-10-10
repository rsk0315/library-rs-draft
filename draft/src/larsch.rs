use std::ops::Add;

/// LARSCH algorithm。
///
/// $A\_{i, j} = f(E\[j\]) + w(i, j)$ で定まる行列を考える。ただし、
/// - $E\[0\]$ は given
/// - $E\[i\] = \\min\_{j\\in\\halfco{0}{i}} A\_{i, j}$ ($i\\in\\halfco{1}{n}$)
/// - $f(\\bullet)$ は高速に計算可能
/// - $w$ は concave QI を満たす
///
/// とする。ただし、$A\_{i, j}$ は、$E\[j\]$ を計算した後にアクセス可能になる[^1]。
///
/// [^1]: 実際の LARSCH algorithm は、$E\[i\]$ を求めた後に $C\_i$ の値および列
/// $C\_{i-1}+1, \\dots, C\_i$ がアクセス可能になる、という形式の制約を扱うが、
/// そうした問題設定があまりなさそうなので簡略化。
///
/// これは、特定の DP の高速化と見なすことができる。
/// $E\[i\]$ は $\\DP\[i\]$ に対応する。$A\_{i, j}$ は、状態 $j$
/// まで最適に遷移してから状態 $i$ へ遷移したときのコストに対応する。
///
/// 各 $i\\in\\halfco{1}{n}$ について、$\\argmin\_{j\\in\\halfco{0}{i}} A\_{i, j}$
/// および $\\min\_{j\\in\\halfco{0}{i}} A\_{i, j}$ を求める。
/// 特に、後者が $\\DP\[i\]$ である。
pub fn larsch<T: Add>(
    n: usize,
    f: impl FnMut(&T) -> T,
    w: impl FnMut(usize, usize) -> T,
) -> (Vec<usize>, Vec<T>) {
    todo!();
}
