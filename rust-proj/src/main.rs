// fn main(){
//     let first_name: &str = "Shubham";
//     let last_name: &str = "Gupta";
//     print!("Full name is : {} {}",first_name,last_name);
// }

// 
// fn main(){
//     let num1 : i8 = 5;
//     let num2 : i8 = 114;
//     print!("sum of {} and {} is {}",num1,num2,num1+num2);
// }


// this wil cause error during compilation which states add with overfloww
// fn main(){
//     let mut number : i8 = 5;  //here we introduce new thing 'mut' which is for mutable and let compiler to change value of a variabke during run time
//     for i in 0..1000{
//         number = number + 100;
//     }

//     print!("value of number is  : {}",number);
// }




fn main(){
    let ismale : bool = true;
    let isadult : bool = true;

    if ismale{
        print!("you are a male ");
    }else{
        print!("you are not a male");
    }

    if ismale && isadult{
        print!("you are an adult male");
    }
}