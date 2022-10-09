// セグ木とかを持つのは HL 分解の責務ではない気がするんだよね。
// なので、適当に頂点のマッピングとかと、パスを返すだけにする。
//
// 0 [ --- H --- ] 1 [ L ] 2 [ L ] 3 [ - H - ] 4 [ L ] 5 [ - H - ] 6 [ - H - ] 7
//
// - (0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7)
//     - H-辺または L-辺の列を返す。
//     - 辺属性の値をセグ木に乗せるとき用？
// - (0, 1), (2, 2), (3, 4), (5, 6), (6, 7)
//     - H-辺区切りの列を返す。
//     - 頂点属性の値をセグ木に乗せるとき用？
//
// 前に考えたことがあったはずで、これでよかったかは後で調べる。
//
// あと部分木の区間とかも？
struct Hld {}

impl Hld {
    pub fn new(par: &[usize]) -> Self {
        todo!();
    }

    pub fn path_h_edges(&self, src: usize, dst: usize) -> Vec<(usize, usize)> {
        todo!();
    }

    pub fn path_edges(&self, src: usize, dst: usize) -> Vec<(usize, usize)> {
        todo!();
    }

    pub fn subtree_range(&self, v: usize) -> (usize, usize) {
        todo!();
    }
}
