// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - O(1)での解答は思いつかない

  何を考えて解いていたか
  - Rustが対応してなかったから一番馴染みのあるTypeScriptで解くことにした
  - pos受け取ってないけど問題ないの？ -> Note that pos is not passed as a parameter.
  - headからnodeがループしているかを調べる
  - たどった場所を記録しておいて、2回目に来たらループしているので処理を中断する

  正解してから気づいたこと
  -
*/

class ListNode {
  val: number
  next: ListNode | null
  constructor(val?: number, next?: ListNode | null) {
      this.val = (val===undefined ? 0 : val)
      this.next = (next===undefined ? null : next)
  }
}

const visited = new Set<ListNode>();
export function hasCycle(head: ListNode | null): boolean {
  if (!head) return false;
  if (visited.has(head)) return true;
  visited.add(head);
  return hasCycle(head.next);
};
