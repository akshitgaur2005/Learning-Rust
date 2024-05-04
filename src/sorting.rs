use std::io;

fn main() {
    println!("Please input how many numbers you want!");

    let array_num = get_int();
    let mut store: Vec<u32> = Vec::new();

    for _ in 0..array_num {
        let x = get_int();
        store.push(x);
    };

    // bubble_sort(&mut store);
    store = merge_sort(&store);

    println!("{:?}", store);
}

fn get_int() -> u32 {
    loop {
        let mut x = String::new();

        io::stdin().read_line(&mut x).expect("Failed to read line");

        match x.trim().parse::<u32>() {
            Ok(num) => {return num},
            Err(_) => true,
        };

        println!("Please enter the number again");
    };
}

fn bubble_sort(store: &mut Vec<u32>) {
    let mut tot_ops: u32;
    loop {
        tot_ops = 0;
        for i in 0..(store.len() - 1) {
            let prev = &store[i];
            let next = &store[i + 1];

            if prev > next {
                store.swap(i, i+1);
                tot_ops += 1;
            };
        };

        if tot_ops == 0 {
            break;
        };
    };
}

fn merge_sort(store: &Vec<u32>) -> Vec<u32> {
    let new_store = store.clone();
    let len = store.len();
    if len <= 1 {
        return new_store;
    } else {
        let mid = len / 2;
        let left = merge_sort(&new_store[0..mid].to_vec());
        let right = merge_sort(&new_store[mid..].to_vec());
        return merge(&left, &right);
    }
}

fn merge(left: &Vec<u32>, right: &Vec<u32>) -> Vec<u32> {
    let mut store: Vec<u32> = Vec::new();
    let mut left = left.clone();
    let mut right = right.clone();
    
    while !(left.is_empty() && right.is_empty()) {
        let n = left.get(0).cloned().unwrap_or(std::u32::MAX);
        let m = right.get(0).cloned().unwrap_or(std::u32::MAX);

        if n < m {
            store.push(n);
            left.remove(0);
        } else {
            store.push(m);
            right.remove(0);
        }
    }

    store
}
