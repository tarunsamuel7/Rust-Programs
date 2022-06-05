use std::collections::HashMap;
use std::io;
fn main() {
    
    let mut v = Vec::new();
    //let mut v = vec![21,49,28,7,10,55,31,12,21,7,7];

    println!("Please enter list size.");
    let mut size = String::new();
    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read line");
    
    let size: u32 = match size.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not valid size")
    };

    for i in 0..size {
        println!("Please input element {}.", i+1);

        let mut input_value = String::new();
        io::stdin()
            .read_line(&mut input_value)
            .expect("Failed to read line");

        let input_value: u32 = match input_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        v.push(input_value);
    }
    
    v.sort();
    println!("Vector: {:?}", v);

    let length = v.len();
    println!("Length: {}", length);

    if length % 2 == 1 {
        println!("Median: {}", &v[(length/2)]);
    }else{
        println!("Median: {}", (&v[(length/2)] + &v[(length/2)-1])/2);
    }

    let mut hmap = HashMap::new();
    for i in &mut v {
        //println!("{}", i);
        let count = hmap.entry(i).or_insert(0);
        *count += 1;
    }
    println!("Map: {:?}", hmap);

    // let highest_element = example(&hmap);
    let highest_element = example(&hmap);


    match highest_element {
        None => println!("Mode: None"),
        Some(i) => println!("Mode: {}", i),
    }
}

fn example<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}
