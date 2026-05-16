// shadowing
//
fn main(){
    let mut x: i32 = 12;
    {
        println!("OG x is {}",x);
        let x: i32 = 1;
        println!("shadowing x is {}",x);
    }
    x = 20;
     println!("2 OG x is {}",x);

}
