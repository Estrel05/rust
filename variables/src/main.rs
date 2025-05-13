fn main() {
    /* mut
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    */

    /* const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    */

    // shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    /* Difference between shadowing and mut
    let spaces = "    ";
    let spaces = spaces.len();
    => possible

    let mut spaces = "    ";
    spaces = spaces.len();
    => impossible

    shadowing => possible to change type
    mut => impossible to change type
    */
}
