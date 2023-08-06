

use std::io::stdin;//use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    
    //vvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    //|| Variable and Data Types ||
    //^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    
    // mut : imutable or value changable 
    
    let mut a:i32=3;
    println!("a value:{}",a);
    a=5;
    println!("{}",a);
    //
    //PRIMITIVE DATA TYPES
    //
    //unsigned integer (only possitive numbers?)
    //u8,u16,u32,u64,u123
    let unsigned:u8=10; //what's the upper bound of unsigned number?
    //signed integer
    //i8,i16,i32,64,i128
    let signed:i8=10;
    //float is used for decimal
    let float:f32=1.2;
    //char
    let letter:char='a'; //cannot use double quote marks "a" why?
    //bool
    let is_true:bool=true;
    
    //array
    let arr: [i32;5] = [1,2,3,4,5];
    
    //turple
    let tur: (i32,f64,u8) = (-500,6.4,1);

    let (x,y,z) = tur;
    //same as following syntax:
    //let x:i32=tur.0;
    //let y:f64=tur.1;
    //let z:u8=tur.2;
    
    //let (x: i32,y: f64,z: u8) = tur; why not working?

    println!("unsigned:{}, signed:{}, float:{}, letter:{}, is_true:{}",unsigned,signed,float,letter,is_true);
    println!("index:{}, length:{}",arr[0],arr.len());
    println!("{:?}",arr);
    
    println!("x:{},y:{},z:{}",x,y,z);

    //vvvvvvvvvvvvvv
    //|| Function ||
    //^^^^^^^^^^^^^^
    
    another_function();
    
    //Put values into a function
    //function_a( x:5, alphabet:'a' );
    functionA( 5, 'a' );
   
    //function?
    let x_tmp1: ()={
       let mut y_tmp1: i32=5 ;
       y_tmp1=y_tmp1+1;
    };
    println!("{:?}",x_tmp1);


    //vvvvvvvvvvvvvvvvvv
    //|| Control Flow ||
    //^^^^^^^^^^^^^^^^^^

    let x: i32 =3;
    if x>5{
        println!("x is greater than 5");
    }
    else if x<2{
        println!("x is less than 2");
    }
    else{
        println!("x is between 2 and 5");
    }
   
    //vvvvvvvvvvvvvvvvvvv
    //|| Random number ||
    //^^^^^^^^^^^^^^^^^^^

    println!("Guessing a number");
    let secret_number:u32=rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please guess a number:");
        let mut guess: String=String::new();
        //io::stdin() Stdin
        //    .read_line(buf:&mut guess) Result<usize, Error>
        //    .expect(msg:"failed to read the line");
        stdin().read_line(&mut guess).expect("failed to read the line");

        //Check if guess = no then skip entire loop!
        //Q: Why this checking need to be below stdin()...;
        if guess.trim()=="no"{
            break;
        }
        //let guess: u32 = match guess.trim().parse(){
        //    Ok(num: u32) => num,
        //    Err(_) => continue,
        //};
        let guess: u32 = match guess.trim().parse(){
            Ok(u32) => u32,
            Err(_) => continue,
        };
        println!("Your guessed:{}",guess);
        if guess>secret_number{
            println!("too big!");
        }
        else if guess<secret_number {
            println!("too small!");
        }
        else{
            println!("correct!");
            break;
        }
    }
    
    //vvvvvvvvvv
    //|| Loop ||
    //^^^^^^^^^^

    //loop
    let mut loop1=0;
    loop{
        if loop1 > 10{
            break;
        } 
        else {
            println!("{}",loop1);
            loop1=loop1+1;
        }
    }

    //while
    let mut loop2=0;
    while loop2<=10{
        println!("{}",loop2);
        loop2+=1;
    }

    //for
    let numarr_tmp1=[10,20,30,40,50];
    for loop3 in 0..numarr_tmp1.len(){
        println!("{}",numarr_tmp1[loop3]);
    }

    //stack memory: u8, i8, float
    //heep memory: strings,vector, etc

}



fn another_function() {
    println!("another function");
}

fn functionA(x:i32,alphabet:char) {
    println!("functionA x:{},alphabet:{}",x,alphabet);
}
