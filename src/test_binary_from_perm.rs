
// use std::time::Instant;

// const N: usize = 16;


// fn main() {
//     let start = Instant::now();
//     let permutation: Vec<u8> = (0..16).collect();
//     let mut binary: Vec<u8> = vec![];
//     for i in 0..N{
//         for j in i..N{
//             if permutation[i]>permutation[j]{
//                 binary.push(0);
//             }
//             else {
//                 binary.push(1);
//             }
//         }
//     }
//     println!("{:.3}",binary.len());
//     println!("Array iteration time: {:?}", start.elapsed());
// }


use std::time::Instant;

const N: usize = 16;

fn fast_approach(permutation: &[usize]) -> Vec<usize> {
    let mut binary = vec![0; N * (N - 1) / 2];
    let mut count = vec![0; N];

    for &x in permutation.iter() {
        for i in (x + 1)..N {
            count[i] += 1;
        }
    }

    let mut index = 0;
    for i in 0..N {
        for j in (i + 1)..N {
            binary[index] = count[j];
            index += 1;
        }
    }

    binary
}

fn main() {
    let start = Instant::now();
    let permutation: Vec<usize> = (0..N).collect();
    let binary = fast_approach(&permutation);
    println!("{:?}", binary);
    println!("Fast approach time: {:?}", start.elapsed());

    let start = Instant::now();
    let mut binary: Vec<u8> = vec![];
    for i in 0..N {
        for j in i+1..N {
            if permutation[i] > permutation[j] {
                binary.push(0);
            } else {
                binary.push(1);
            }
        }
    }
    println!("{:.3}", binary.len());
    println!("Array iteration time: {:?}", start.elapsed());
}
