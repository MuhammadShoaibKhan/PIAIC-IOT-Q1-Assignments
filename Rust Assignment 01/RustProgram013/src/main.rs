//Store an integer (any value) in a variable and ​print the table
//of that number using any loop

fn main() {
    let mut num = 5;
    for mut x in (1..11){

       println!("{} * {} = {}",num, x, num*x); // x: i32

    }
}
