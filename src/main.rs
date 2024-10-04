fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let index = 2;

    println!("Original vec: {:?}", vec);
    vec.remove(index);
    println!("Vec after removing {} element, {:?}", index, vec);
}
