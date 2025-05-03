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
  他の人のコードを読んで考えたこと
  - https://github.com/nktr-cp/leetcode/pull/5/files
  これと同じコードをRustで書こうとするとすごく長くなってしまう
  - https://github.com/Yoshiki-Iwasa/Arai60/blob/main/problems/src/remove_duplicates_from_sorted_list_ii/step2.rs

  改善する時にかんがえたこと
  -
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
        let sentinel = ListNode {
            val: i32::MIN,
            next: head,
        };
        let mut prev = Box::new(sentinel);
        fn delete_node_recursive(prev_node: &mut Box<ListNode>) {
            let current = &mut prev_node.next;
            if current.is_none() {
                return;
            }
            let current_node = current.as_mut().unwrap();

            let mut is_duplicated = false;
            while let Some(next_node) = current_node.next.as_mut() {
                if next_node.val != current_node.val {
                    break;
                }
                is_duplicated = true;
                current_node.next = next_node.next.take();
            }

            if is_duplicated {
                prev_node.next = current_node.next.take();
                delete_node_recursive(prev_node);
            } else {
                delete_node_recursive(current_node);
            }
        }

        delete_node_recursive(&mut prev);

        prev.next
    }
}
// @lc code=end
