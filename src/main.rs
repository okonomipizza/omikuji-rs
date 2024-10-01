use rand::prelude::*;

fn main() {
    let omikujis = ["大吉", "吉", "中吉", "小吉", "末吉", "凶", "大凶"];

    // 乱数生成器
    let mut rng = rand::thread_rng();

    // omikujis 配列の長さの範囲でランダムな index を作成
    let random_index = rng.gen_range(0..omikujis.len());

    let selected_value = omikujis[random_index];

    println!("今日のあなたの運勢は '{}' です！", selected_value);
}
