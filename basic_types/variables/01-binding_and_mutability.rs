fn main(){
    let x: i32 = 5; //uninitialised but used, Error!
    let _y: i32; // Uninitialised and unused, only a Warning

    assert_eq!(x,5);
    println!("Success"); // Prints when the program is successful
}
