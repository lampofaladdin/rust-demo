// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert("Blue", 10);

//     // 覆盖已有的值
//     let old = scores.insert("Blue", 20);
//     assert_eq!(old, Some(10));

//     // 查询新插入的值
//     let new = scores.get("Blue");
//     assert_eq!(new, Some(&20));

//     // 查询Yellow对应的值，若不存在则插入新值
//     let v = scores.entry("Yellow").or_insert(5);
//     assert_eq!(*v, 5); // 不存在，插入5

//     // 查询Yellow对应的值，若不存在则插入新值
//     let v = scores.entry("Yellow").or_insert(50);
//     assert_eq!(*v, 5); // 已经存在，因此50没有插入
// }

// use std::collections::HashMap;
// fn main() {
//     let text = "hello world wonderful world";

//     let mut map = HashMap::new();
//     // 根据空格来切分字符串(英文单词都是通过空格切分)
//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", map);
// }

// // 填空并修复错误
// use std::collections::HashMap;
// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert("Sunface", 98);
//     scores.insert("Daniel", 95);
//     scores.insert("Ashley", 69);
//     scores.insert("Katie", 58);

//     // get 返回一个 Option<&V> 枚举值
//     let score = scores.get("Sunface");
//     assert_eq!(score, Some(&98));

//     if scores.contains_key("Daniel") {
//         // 索引返回一个值 V
//         let score = scores["Daniel"];
//         assert_eq!(score, 95);
//         scores.remove("Daniel");
//     }

//     assert_eq!(scores.len(), 3);

//     for (name, score) in scores {
//         println!("The score of {} is {}", name, score)
//     }
// }

// use std::collections::HashMap;
// fn main() {
//     let teams = [
//         ("Chinese Team", 100),
//         ("American Team", 10),
//         ("France Team", 50),
//     ];

//     let mut teams_map1 = HashMap::new();
//     for team in &teams {
//         teams_map1.insert(team.0, team.1);
//     }

//     // 使用两种方法实现 team_map2
//     // 提示:其中一种方法是使用 `collect` 方法
//     let teams_map2: HashMap<&str, i32> = teams.into_iter().collect();

//     assert_eq!(teams_map1, teams_map2);

//     println!("Success!");
//     println!("{:?}", teams)
// }

// 填空
use std::collections::HashMap;
fn main() {
    // 编译器可以根据后续的使用情况帮我自动推断出 HashMap 的类型，当然你也可以显式地标注类型：HashMap<&str, u8>
    let mut player_stats = HashMap::new();

    // 查询指定的 key, 若不存在时，则插入新的 kv 值
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);

    // 通过函数来返回新的值
    player_stats
        .entry("health")
        .or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(*health, 100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("Success!")
}

fn random_stat_buff() -> u8 {
    // 为了简单，我们没有使用随机，而是返回一个固定的值
    42
}
