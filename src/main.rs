use std::collections::HashMap;

fn main() {
    let data: [HashMap<&str, i32>; 8] = [
        HashMap::from([("a", 3)]),
        HashMap::from([("b", 5)]),
        HashMap::from([("c", 3)]),
        HashMap::from([("d", 9)]),
        HashMap::from([("a", 0)]),
        HashMap::from([("b", 2)]),
        HashMap::from([("c", 7)]),
        HashMap::from([("u", 8)]),
    ];

    let mut result_hash_map = HashMap::new();

    for element in data {
        for (key, val) in element.iter() {
            match result_hash_map.get(key) {
                Some(val_from_res) => {
                    result_hash_map.insert(key.clone(), val + val_from_res);
                }
                None => {
                    result_hash_map.insert(key.clone(), val.clone());
                }
            };
        }
    }
    println!("{:?}", result_hash_map);
}
