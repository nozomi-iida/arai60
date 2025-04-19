// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - RustでListNodeの扱い方が分からないな...
  - ListNodeを削除する方法が分からなかった
  https://leetcode.com/problems/remove-duplicates-from-sorted-list/solutions/5810798/video-explain-important-points/
  https://leetcode.com/problems/remove-duplicates-from-sorted-list/solutions/1557910/rust-simple-concise-iterative-0ms-100/
  知らないRustの文法がでてきた
  Box: ヒープメモリを管理するためのスマートポインタ。サイズが決まっていない型や、再帰的なデータ構造に使われる
    ヒープメモリ: 動的に確保されるメモリ領域のこと
    スマートポインタ: ポインタの役割をもちつつ、メモリ管理の一部を自動化する構造体。通常のポインタは所有権や解放の管理を手動で行う必要があるが、Rustの場合、スマートポインタはスコープを抜けると自動的に解放される
      ポインタ: メモリ内のどこにデータが保持されているかを示すもの
  as_mut: Optional<T>をOptional<&mut T>に変換するメソッド
  take: OptionalでSome(value)の場合にvalueを取り出すメソッド。Noneの場合はNoneを取り出す。ミュータブルな値にしか使えない

  なんでas_mut()で可変参照したものを操作するの？直接操作すれば良くない？
  `val`を変更したいときに、`head.val = 99;`と書くとコンパイルエラーになり、Optionalの値の中に入るには`unwrap`か、`as_ref/as_mut`などで借用する必要がある
  ここは何回も書いて肌で覚えるところかなと感覚的に思ったから、ここで調査終了

  何を考えて解いていたか
  - ソートされたnodeが渡されて、ダブったnodeを削除する問題
  - ソートされてるから、ダブったnodeは隣にあるはず

  正解してから気づいたこと
  -
*/

/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
 */

// @lc code=start
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current_option = head.as_mut();
        while let Some(current) = current_option {
            let mut next_option = current.next.take();
            while let Some(next) = next_option.as_mut() {
                if current.val != next.val {
                    current.next = next_option;
                    break;
                }
                next_option = next.next.take();
            }
            current_option = current.next.as_mut();
        }
        head
    }
}
// @lc code=end
