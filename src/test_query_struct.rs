use std::time::Instant;

fn remove_and_print_v1(lst: &[u8], elem: u8) {
    for i in 0..lst.len() {
        let mut copy = lst.to_vec();
        copy.remove(i);
    }
}

fn remove_and_print_v2(lst: &[u8]) {
    for i in 0..lst.len() {
        let copy: Vec<u8> = lst
            .iter()
            .enumerate()
            .filter(|&(j, _)| i != j)
            .map(|(_, &x)| x)
            .collect();
    }
}

fn remove_and_print_v3(lst:&[u8]){
    for elem in lst.iter(){
        let forward:[u8] = lst.iter().filter(|&x| x == elem);
    }
}



fn main() {
    let lst = vec![0b0001, 0b0010, 0b0100, 0b1000];

    let start = Instant::now();
    for _ in 1..1_000_000{
        remove_and_print_v1(&lst, 0);
    }
    let elapsed = start.elapsed();
    println!("v1: {:?}", elapsed);


    let start = Instant::now();
    for _ in 1..1_000_000{
        remove_and_print_v2(&lst);
    }
    let elapsed = start.elapsed();
    println!("v2: {:?}", elapsed);

    let start = Instant::now();
    for _ in 1..1_000_000{
        remove_and_print_v3(&lst);
    }
    let elapsed = start.elapsed();
    println!("v3: {:?}", elapsed);
}
