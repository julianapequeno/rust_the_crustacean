fn main() {
    let x = 5;

    let x = x + 1; //shadowing

    {//scope
        let x = x * 2; //shadowing
        println!("The value of x in the inner scope is: {x}");
    } //the 2nd x die here.

    println!("The value of x is: {x}");

    //Unsigned are only positive numbers
    //Signed can be positive as well as negative
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let index = a[0];
    println!("Heii {index}");
}