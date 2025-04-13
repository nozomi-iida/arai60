// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - O(n^2)の解法しか思いつかなかった

  正解してから気づいたこと
  - https://leetcode.com/problems/two-sum/ を見ながら実装した
  - target - nums[i]で残りの数を求めて、それをkeyとするhashのvalueを答えにしているのか

  - 一回のループで解く方法も紹介されていた
  - 最初に合計でtargetになる数があるかをチェックして、なかったらhashに追加していくことで、1回のforループで解いている
*/

pub struct Solution;

/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;
// 2回forループを回す
// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         let mut hash = HashMap::new();
//         for i in 0..nums.len() {
//             hash.entry(nums[i]).or_insert(i);
//         }
//         for i in 0..nums.len() {
//             let complement = target - nums[i];
//             if let Some(&j) = hash.get(&complement) {
//                 if i != j {
//                     return vec![i as i32, j as i32];
//                 }
//             }
//         }

//         vec![]
//     }
// }
// 1回forループを回す
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash = HashMap::new();
        for i in 0..nums.len() {
            let complement = target - nums[i];
            if let Some(&j) = hash.get(&complement) {
                if i != j {
                    return vec![i as i32, j as i32];
                }
            }

            hash.entry(nums[i]).or_insert(i);
        }

        vec![]
    }
}
// @lc code=end
