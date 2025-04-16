// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(1)
  空間計算量: O(1)
*/


// 初期コード
class ListNode {
  val: number
  next: ListNode | null
  constructor(val?: number, next?: ListNode | null) {
    this.val = (val===undefined ? 0 : val)
    this.next = (next===undefined ? null : next)
  }
}

export function detectCycle(head: ListNode | null): ListNode | null {
  let slow = head;
  let fast = head;
  while (true) {
    if (!fast || !fast.next) {
      return null;
    } 
    slow = slow?.next!;
    fast = fast.next.next;
    if (fast === slow) {
      break;
    }
  }
  slow = head;
  while (fast) {
    if (slow === fast) {
      return fast;
    } 
    slow = slow?.next!;
    fast = fast.next;
  }
  return null;
};
