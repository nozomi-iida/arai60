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
  https://github.com/Yoshiki-Iwasa/Arai60/blob/main/problems/src/two_sum/step3.rs
  - 1回のforループで解いている
  - ifではなく、matchを使っている()
  - eprintlnで答えがないときに標準エラーを出力している
  - enumerateを使ってindexと値を同時に取得している
  - complimentって誉めるって単語で、、正しくはcomplement(補数)らしい

  https://github.com/carolina-museum/coding-challenges/pull/1/files
  - 2分探索を使った解放があった
    2文探索で、2つの数を出すとtargetになる値があるかを調べている


  改善する時にかんがえたこと
  > You may assume that each input would have exactly one solution
  回答は1つあると問題文に書いてあったので、回答が見つからない場合はpanic!使ってエラーにした
*/

pub struct Solution;

/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            match map.get(&complement) {
                Some(&j) => {
                    return vec![i as i32, j as i32];
                }
                None => {
                    map.insert(num, i);
                }
            }
        }
        panic!(
            "No pairs can make the target. nums: {:?}, target: {:?}",
            nums, target
        )
    }
}
// @lc code=end
