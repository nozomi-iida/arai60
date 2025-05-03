// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - current_nodeの扱い方が間違えてると思う
  https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/solutions/6667486/c-beats-100-simple-code-easy/
  -

  何を考えて解いていたか
  - 複数あった場合その数字は削除するってこと
  1. current_nodeとnext_nodeと、current_nodeが被っているかのフラグを持つ
  2. current_nodeとnext_nodeの値を比べる
    - 異なる場合
      - フラグがtrueの場合は、current_nodeにnext_nodeを渡す
      - フラグがfalseの場合は答えのcurrent_nodeにnext_nodeのnextを渡す
      1を繰り返す
    - 同じ場合はnext_nodeのさらにnextをとり、current_nodeが被っているかのフラグをtrueにする
  3. headを返す

  正解してから気づいたこと
  1. headが削除されることがあるのでdummy_nodeを先頭にしたanswerを用意する
  2. current_nodeとnext_nodeと、current_nodeが被っているかのフラグを持つ
  3. current_nodeとnext_nodeの値を比べる
    - 異なる場合
      1. is_duplicateをtrueにする
      2. current.nextをnext.nextに変更する
      3. 2を繰り返す
    - 同じ場合
      - フラグがtrueの場合
        previous_node.nextをcurrent.nextにする
      - フラグがfalseの場合

*/

/*
 * @lc app=leetcode id=82 lang=rust
 *
 * [82] Remove Duplicates from Sorted List II
 */

pub struct Solution;
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut answer = Box::new(ListNode { val: 0, next: head });
        let mut previous_node = &mut answer;
        while let Some(mut current_node) = previous_node.next.take() {
            let mut is_duplicate = false;
            while let Some(next_node) = current_node.next.as_mut() {
                if current_node.val != next_node.val {
                    break;
                }
                is_duplicate = true;
                current_node.next = next_node.next.take()
            }
            if is_duplicate {
                previous_node.next = current_node.next;
            } else {
                previous_node.next = Some(current_node);
                if let Some(ref mut node) = previous_node.next {
                    previous_node = node;
                }
            }
        }
        answer.next
    }
}
// @lc code=end
