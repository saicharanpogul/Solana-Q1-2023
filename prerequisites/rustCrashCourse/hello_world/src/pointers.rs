pub fn run() {
    // primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("array: {:?}", (arr1, arr2));

    // vectors 
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("vectors: {:?}", (&vec1, vec2));
}