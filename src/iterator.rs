fn main() {
    let original_vec: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

    let new_vec: Vec<char> = original_vec
        .iter() // the iterator
        .skip(1) // skip 'a'
        .map(|&c| c) 
        .chain(std::iter::once('f')) // add 'f'
        .collect(); 

    println!("{:?}", new_vec);
}
