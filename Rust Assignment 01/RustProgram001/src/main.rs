fn main() {


    //Store a string ​“PAKISTAN ZINDABAD”​ in a variable, also
    //calculate the length of string and store it in a variable​. ​Print
    //both variables.
    println!("Program Output 01");

    let a = "PAKISTAN ZINDABAD";
    let length = a.len();
    println!("{:?}",a);
    println!("LENGTH OF PAKISTAN ZINDABAD IS {}",length);

    //Store ​85​ in a variable if it is an unsigned 64-bit number. Also
    //store ​-550 ​in a variable​ ​if it is​ ​16-bit integer. Print both
    //variables.
    println!("\n\nProgram Output 02");
    let num1:u64 = 85;
    let num2:i16 = -550;

    println!("{}",num1);
    println!("{}",num2);

    //Store a float 56.6 in a variable if it is a 32-bit float also print
    //the variable.

    println!("\n\nProgram Output 03");
    let b:f32 = 56.6;
    println!("{}",b);

    //Suppose there are two integer numbers ​76​ and ​23. ​Store
    //both of them in variables with any name and apply following
    //operations on it.
    println!("\n\nProgram Output 04");
    let a1 = 76;
    let a2 = 23;

    println!("x + y = {}",a1 + a2);
    println!("x - y = {}",a1 - a2);
    println!("x * y = {}",a1 * a2);
    println!("x / y = {}",a1 / a2);
    println!("x % y = {}",a1 % a2);

    //Store following values in an array. Print the array and also print
    //the values ​150​ and ​250

    println!("\n\nProgram Output 05");

    let array_1 = [100, 150, 200, 250, 300];
    println!("{:?}",array_1);
    println!("{}", array_1[1]);
    println!("{}", array_1[3]);

    //Store following values in a tuple​. ​Print the tuple and also print
    //cloud​, ​8645​ and ​65.4​.

    println!("\n\nProgram Output 06");
    let tuple_a = ("IoT","AI","Cloud",500.65, 8645, 65.4);
    println!("{:?}",tuple_a);
    println!("{}",tuple_a.2);
    println!("{}",tuple_a.4);
    println!("{}",tuple_a.5);


}
