// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  -

  何を考えて解いていたか
  - tailはどのこindexとつながっているかを解答する問題
  - 前の問題と同じように2つのポインタで解くことはできないのかな？
  - 前の問題だとどこかで2つのポインタが同じになることでループかどうかを判断していたけど、今回の問題はループのポイントを求めるからちょっと違う
  - 43行目のcurに`head.next`を渡してたり、returnで`cur`を返したりコード書く前の整理がたりなくて、ミスがあった

  正解してから気づいたこと
  - 空間計算量をO(1)で解く方法は分からなかった
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
  const visited = new Set();
  visited.add(head);
  let cur = head;
  while (cur?.next) {
    if (visited.has(cur.next)) {
      return cur.next;
    } else {
      visited.add(cur.next);
      cur = cur?.next ?? null;
    }
  }
  return null;
};
