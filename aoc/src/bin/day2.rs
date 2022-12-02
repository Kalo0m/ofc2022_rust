use std::collections::HashMap;

fn get_hash_table_part_1() -> HashMap<&'static str, i32> {
    let mut hash_table = HashMap::new();
    hash_table.insert("A X", 3 + 1);
    hash_table.insert("A Y", 6 + 2);
    hash_table.insert("A Z", 0 + 3);
    hash_table.insert("B X", 0 + 1);
    hash_table.insert("B Y", 3 + 2);
    hash_table.insert("B Z", 6 + 3);
    hash_table.insert("C X", 6 + 1);
    hash_table.insert("C Y", 0 + 2);
    hash_table.insert("C Z", 3 + 3);
    return hash_table;
}

fn get_hash_table_part_2() -> HashMap<&'static str, i32> {
    let mut hash_table = HashMap::new();
    hash_table.insert("A X", 0 + 3);
    hash_table.insert("A Y", 3 + 1);
    hash_table.insert("A Z", 6 + 2);
    hash_table.insert("B X", 0 + 1);
    hash_table.insert("B Y", 3 + 2);
    hash_table.insert("B Z", 6 + 3);
    hash_table.insert("C X", 0 + 2);
    hash_table.insert("C Y", 3 + 3);
    hash_table.insert("C Z", 6 + 1);
    return hash_table;
}

fn main() {
    let hash_table_part1 = get_hash_table_part_1();
    let hash_table_part2 = get_hash_table_part_2();

    let lines: Vec<&str> = include_str!("./day2.txt").lines().collect();

    let part1: i32 = lines.iter().map(|l| hash_table_part1.get(l).unwrap()).sum();
    let part2: i32 = lines.iter().map(|l| hash_table_part2.get(l).unwrap()).sum();

    println!("partie 1 {:?}\npartie 2 {:?}", part1, part2);
}
