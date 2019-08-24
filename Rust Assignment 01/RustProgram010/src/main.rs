// Store year in variable like this ​(let year = 2019;) ​and check if
//the year is leap or not


fn main() {
    let year = 2020;

    if(year%4 == 0)
    {
        if( year%100 == 0)
        {
            // year is divisible by 400, hence the year is a leap year
            if ( year%400 == 0){
                println!("{} is a leap year.", year);
            }

            else{
                println!("{} is not a leap year.", year);
            }

        }
        else{
            println!("{} is a leap year.", year );
        }

    }
    else{
        println!("{} is not a leap year.", year);

    }


}
