// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時にかんがえたこと

/*
  講師陣はどのようなコメントを残すだろうか？
  -

  他の人のコードを読んで考えたこと
  https://github.com/katsukii/leetcode/pull/19/files
  https://github.com/Satorien/LeetCode/pull/2/files
  - 実装に手一杯で、他の人のコードをみて考える余裕があまりなかったが、書いているコードは自分のコードとあまり違いはないように感じた

  改善する時にかんがえたこと
  - current_nodeのほうが適切だと思った
  - Rustを使っているからか、変数の所有権を考えないといけなくて、他のコードよりも複雑だと感じた
  - 2つのポインタを扱う感覚がつかめてない
*/

/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
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
        let mut current_node = head.as_mut();
        while let Some(current) = current_node {
            let mut next_node = current.next.take();
            while let Some(next) = next_node.as_mut() {
                if current.val != next.val {
                    current.next = next_node;
                    break;
                }
                next_node = next.next.take();
            }
            current_node = current.next.as_mut();
        }
        head
    }
}
// @lc code=end
