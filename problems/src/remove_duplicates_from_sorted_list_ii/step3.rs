// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(n)
  空間計算量: O(1)
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
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut previous_node = &mut dummy;
        while let Some(mut current_node) = previous_node.next.take() {
            let mut is_duplicate = false;
            while let Some(next_node) = current_node.next.as_mut() {
                if current_node.val != next_node.val {
                    break;
                }
                is_duplicate = true;
                current_node.next = next_node.next.take();
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
        dummy.next
    }
}
// @lc code=end
