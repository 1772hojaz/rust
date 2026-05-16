fn main(){
    //A scope is the range within the program for which the iterm is valid
    let x: i32 = 10;
    {
        let y:i32 = 5;
        println!("The value of x is  {} and the value of y is {}", x, y);
    }// This is the inner scope

    println!("the value of x is {} ", x)

}//This is the outer scope
