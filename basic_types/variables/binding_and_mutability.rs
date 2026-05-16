fn main(){
    let x: i32; //uninitialised but used, Error!
    let x: i32; // Uninitialised and unused, only a Warning

    assert_eq(x,5);
    println!(Success);
}
