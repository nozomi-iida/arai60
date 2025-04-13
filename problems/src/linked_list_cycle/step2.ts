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
  https://leetcode.com/problems/linked-list-cycle/solutions/6263676/best-solution-ever-python-java-c-c-javascript-go-c-kotlin-typescript-swift/
  https://github.com/Satorien/LeetCode/pull/1/files
  - 1つずつ進むslowと、2つずつ進むfastの2つのポインタを使用している
  - サイクルが存在するなら、2つのポインタがどこかで一致する
  - fastがnullになったら、サイクルは存在しないということ

  改善する時にかんがえたこと
  - 2つのポインターで書くとどこが優れてるんだろう？
  Chat GPTの解答
  1. 空間計算量がO(1)で済む(Setを使っていないため)
  2. グローバル変数を使わなくて良いから、処理をカプセル化できている
*/

class ListNode {
  val: number
  next: ListNode | null
  constructor(val?: number, next?: ListNode | null) {
      this.val = (val===undefined ? 0 : val)
      this.next = (next===undefined ? null : next)
  }
}

export function hasCycle(head: ListNode | null): boolean {
  let slow = head;
  let fast = head;
  while (fast && fast.next) {
    slow = slow?.next!; 
    fast = fast.next.next!;
    if (slow === fast) {
      return true;
    }
  }
  return false;
};
