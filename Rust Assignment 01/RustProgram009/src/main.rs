//Store marks in a variable and print the Grade of student.

fn main() {
    let marks = 35;


    if marks > 80{
        println!("Your grade is A+");
    }
    if marks >= 70 && marks < 80{
        println!("Your grade is A");
    }

    if marks >= 60 && marks < 70{
        println!("Your grade is B");
    }

    if marks >= 50 && marks < 60{
        println!("Your grade is C");
    }

    if marks >= 40 && marks < 50{
        println!("Your grade is D");
    }
    if marks <40{
        println!("You are fail");
    }
}
