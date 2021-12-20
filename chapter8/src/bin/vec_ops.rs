use std::collections::HashMap;

fn main() {
    let a_list = vec![2,9,20,3];
    println!("List: {:?}", a_list);
    println!("Average: {:?}", average(&a_list));
    println!("Mean: {:?}", median(&a_list));
    let mode = mode(&a_list);
    match mode {
        Some(x) => println!("Mode: {}", x),
        None => println!("Empty list!")
    }
}

fn average(list: &Vec<i32>) -> f64 {
    let mut average: f64 = 0.0;
    let mut size: u32 = 0;
    for element in list {
        average += f64::from(*element);
        size += 1;
    }
    average / f64::from(size)
}

fn median(list: &Vec<i32>) -> f64 {
    let mut cl = list.clone();
    cl.sort_unstable();
    if cl.len() % 2 == 1 {
        cl[(cl.len() as f64 / 2.0).floor() as usize] as f64
    }
    else {
        (cl[(cl.len() as f64 / 2.0).floor() as usize] + cl[(cl.len() as f64 / 2.0).ceil() as usize]) as f64 / 2.0
    }
}

fn mode(list: &Vec<i32>) -> Option<i32> {
    let mut map: HashMap<i32, u32> = HashMap::new();

    for e in list{
        let count = map.entry(*e).or_insert(0);
        *count += 1;
    }
    let mut t: Vec<(i32, u32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
    t.sort_unstable_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap());
    t.last().and_then(|x| Some(x.0))
}