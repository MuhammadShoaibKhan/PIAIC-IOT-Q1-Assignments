//Write a function that can take 3 ​float​ numbers as
//argument/parameter and ​returns​ the multiplication result of them.

fn main() {
  let ans =  multiply(5.6,2.4,10.2);
    println!("{}",ans);
}

fn multiply(a:f64, b:f64, c:f64) -> f64{
    let ans:f64 = a*b*c;
    return ans;
}