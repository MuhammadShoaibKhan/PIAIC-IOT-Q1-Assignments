//Write a function that can take 3 ​integer​ numbers as
//argument/parameter and ​prints​ the sum of them

fn main(){
    println!("\n\nProgram Output 07");
    let ans_sum = add(10,20,30);
    println!("{}",ans_sum);
}

fn add(x:i32,y:i32,z:i32) ->i32
{
  let sum = x + y + z;
    return sum;
}

