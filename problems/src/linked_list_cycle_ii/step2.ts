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
  https://leetcode.com/problems/linked-list-cycle-ii/solutions/3274386/c-typescript-javascript-very-easy-solution-in-only-4-lines/
  - 再帰することでO(1)を実現していた
  - 一度訪れたnodeのvalを100500に設定している理由は制約でnode.valの最大値は10^5だから、それ以上の値を設定することでそれ以上の値=すでに訪れているとできる
  https://github.com/shintaro1993/arai60/pull/5/files
  - 2つのポインタでもこの問題も解くことができるんだ
  - 最初にループしているかを確認して、そのあとどこでループしているかを求めている
  ChatGPTに最初の解答と、この解答の比較したら、この解答のほうが優れているらしい
  1. リストの内容を書き変えてない
  2. 前の回答は空間計算量がO(n) (再帰によるスタックの使用)だが、この回答はO(1)しか使わない

  https://github.com/shintaroyoshida20/leetcode/pull/2/files
  https://github.com/katsukii/leetcode/pull/13
  - 2つのポインタでなぜ解けるか深く考察してある

  改善する時にかんがえたこと
  自分でも説明できるようにしてみる
  1. 最初のwhileで与えられたnodeがループするものであることを確定している
  2. そのあと、slowを先頭に戻す
  3. そのあとお互いに1つずつ進んで、同じ地点がnodeのループしている点となる
  3の原理が分からない
  一歩ずつ進んだら衝突しなくない？ -> slowはループの開始地点ではなく、nodeの開始地点に移動する
  もう理解を深めたい
  https://note.com/rhayahi/n/n7fc11c09fec6
  フロイドの循環検出アルゴリズムに関するnoteを呼んでたがいまいち理解できず...
  https://docs.google.com/document/d/11HV35ADPo9QxJOpJQ24FcZvtvioli770WWdZZDaLOfg/edit?tab=t.0
  このコメントでようやく理解できた
  > うさぎとかめは、衝突点で出会った後に、うさぎとかめは、いま来た道を戻るように言われました。うさぎもかめも同じ速さで1歩ずつ歩いて戻ります。このとき、うさぎは一周してから戻りますが、かめはそのまま戻ります。
    かめがスタート地点に戻った時、うさぎはどこにいるでしょうか。実は、うさぎは衝突点にいます。
  ここを図を書いて確認できたときにすっきりした
  言ってることは理解できたけど感覚的に分かっただけで、完全には理解できなかった
  完全に分かるには数学的に理解する必要がありそうだけど、時間がかかりそうなのでここでStopする
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
    // fast.nextでslow.nextがあることも確定しているため
    slow = slow?.next!;
    fast = fast.next.next;
    if (slow === fast) {
      break;
    }
  }
  slow = head;
  while (fast) {
    if (slow === fast) {
      return fast;
    } 
    // 前のwhileでループしていることは確定しているため
    slow = slow?.next!
    fast = fast.next;
  }
  return null;
};
