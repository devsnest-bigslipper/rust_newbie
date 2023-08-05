fn main() {
    println!("Hello, world!");
    
    //vvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    //|| Variable and Data Types ||
    //^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //
    // mut : imutable or value changable 
    //
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
    let tur: (i32,f64,u8) = (500,6.4,1);

    let (x,y,z) = tur;
    //same as following syntax:
    //let x:i32=tur.0;
    //let y:f64=tur.1;
    //let z:u8=tur.2;
    
    //let (x: i32,y: f64,z: u8) = tur; not working?

    println!("unsigned:{}, signed:{}, float:{}, letter:{}, is_true:{}",unsigned,signed,float,letter,is_true);
    println!("index:{}, length:{}",arr[0],arr.len());
    println!("{:?}",arr);
    
    println!("x:{},y:{},z:{}",x,y,z);

    //vvvvvvvvvvvvvv
    //|| Function ||
    //^^^^^^^^^^^^^^
    
    another_function();


}



fn another_function() {
    println!("another function");
}
