fn main() {
    // let x:i32=-4;
    // let y:u32=5;
    // let z:f32=6.0;
    // println!("{} {} {}" ,x, y, z);
     
    // let mut x: bool = false;
    // println!("{}", x);
    // x = true;
    // println!("{}", x);

    // let mut y=4;
    // y=54;
    // println!("{}", y);

    let greetings :String =String::from ("hello vinay");
    println!("{}", greetings);

    let greet : &str ="vinay joshi";
    println!("{}", greet);

    let v =12;
    let j=14;
    if v>j {
        println!("vinay is the great");

    }
    for _ in v..j{
        println!("vinay is the great always");

    }

    let vinay:String=String ::from( "vinay joshi");
    let joshi  :String =getword(vinay);
    println!("{}",joshi);
   fn getword(vinay: String) -> String {
    let mut ans = String::new();
    for c in vinay.chars() {
        if c == ' ' {
            break;
        }
        ans.push(c);
    }
  return   ans;
}
let   pr=5;
change (pr);
fn change(mut kr:i32)->i32
{
    kr+=5;
     println!("{}", kr);
      kr
}
println!("{}", pr);

let mut  gf:String=String::from("kirat");
gf =changestring(gf); // if i make it immutable it will give error , i tried to assign new value to it 

println!("{}", gf);
fn changestring( bf:String)->String
{
    
    return bf;

}
let mut sk:String=String::from("vinay joshi");
update(&mut sk);
println!("{}",sk);
fn update( hk:&mut String){
    hk.push_str(" kyabe lwde");
    println!("{}",hk);


}

struct User{
    name:String,
    age:i8,
    male:bool,
}
 let user1=User{
    name:String::from("vinay"),
    age:21,
    male:true
 };
 println!("{}",user1.name);
 // tuple struct 

 struct Color(u32, u32, u32);
let black= Color(0, 0, 0);
println!("{}", black.0);

//unit struct
struct Marker; // it takes no fields;
let _k =Marker;

struct Rect{
    width:u32,
    height:u32,
}
impl Rect {
    fn area(&self)->u32  //use of &self to refer to the instance of the struct
{
    return  self.width*self.height
}}
 

let rect=Rect{
    width:8,
    height:9,
};
println!("{}",rect.area());

enum Directions{
    North,
    East,
    West,
    South,

}
let my_direction= Directions::West;
println!("{}",my_direction as u8);


enum Shape{
    Circle(f64),
    Square(f64),
}
fn calculate_area(shape:Shape)->f64{
    match shape{
        Shape::Circle(radius)=> {3.14*radius*radius},
        Shape::Square(side)=>{
side*side},    }

}
let my_shape=Shape::Circle(3.22);
let ans=calculate_area(my_shape);
println!("Area: {}", ans);

enum Result<T,E>{
    Ok(T),
    Err(E),
}
 use std::fs;
let result = fs::read_to_string("file.txt");
match result{
    Ok(value)=>{
        println!("Success: {}", value);
    },
    Err(err)=>{
        println!("Error: {}", err);
    }
}

pub enum Option<T>{
    Some(T),
    None,

}

//collections;
let mut vec=Vec::new();
vec.push(1);
vec.push(2);
vec.push(3);

 
    println!("{:?}", vec); // Debug print of the vector
    println!("{}",vec[0]);

    let mut  vec2= vec![1,3,4,5]; // vec! make new vector and Vec::new() and String::new() also do
    for i in vec2.iter_mut(){
       *i+= 1; // This will not change the original vector, as `i` is a reference;
        println!("{}", *i );
    } 
    let mut i=0; 
    while i<vec2.len(){
        if (vec2[i] & 1)==0{
            println!("{}", vec2[i]);
        }
    i += 1;
    };
    
 //Hashmap
 use std::collections::HashMap;
 let mut has_map : HashMap<u32,&str>=HashMap::new();
 has_map.insert(1,"Vinay");
 has_map.insert(2,"Joshi");

    for (key,value) in &has_map {
     
        println!("Key: {}, Value: {}", key, value);
    }
    println!("{:?}", has_map); // Debug print of the HashMap
    let value=has_map.get(&22);
    match value{
        Some(val)=>{
            println!("{}", val);
        }
        None=>{
            println!("Key not found");
        }
    }

    let mut  vec3= vec![2,4,5,6];
    let mut  vec3iter=vec3.iter_mut();
    let  first= vec3iter.next();
    
    match first{
        Some(val)=>{
println!("{}", val);  
        }
        None =>{
            println!("No value found");
        }
    }
    


    
    // iter.next() returns an Option<T> so let Some(val)= iter.next(){

// iter =vec.iter();
// iter.map(|x| x + 1)  // This will not change the original vector, as `x` is a reference;-- this also returns an iterator which we can use futher
//iter.filter(|&x| x % 2 == 0) // This will filter the elements of the vector, returning only even numbers

let mut vec3:Vec<i32>=vec![1,24,5,56,7];
let  vec3_iter=vec3.iter_mut();
let    vec3_filt=vec3_iter.filter(|x| (**x % 2)!=0);
 
  // This will print the filtered elements of the vector
//let mut  vec3filtered:Vec<i32>=Vec::new();
 let mapvec3:Vec<i32>=vec3_filt.map(|x| *x *2).collect(); 
//  for i in mapvec3 {
//     vec3filtered.push(i);




//     }

// println!("{:?}", vec3filtered); // Debug print of the filtered and mapped vector
println!("{:?}", mapvec3); // Debug print of the filtered and mapped vector
println!("{:?}", vec3); // Debug print of the original vector



// strinslices
let str2:String=String::from("vinay joshi");
let s4=&str2[0..5];  // i want 
println!("{s4}");


// traits default implementation if nothing is given
// impl trait for struct {}

trait Summary{
    fn summarize(&self)->String{
       return String::from("Default summary");
    }


}
//impl Summary for String{};

// fn notify (_u :impl Summary){
// }
// notify(String::from("Hello, world!")); // trait as a parameter anything that implements the Summary trait can be passed here

// this is syntactial sugar for trait bounds
// fn notify2<T: Summary>(_u: T) {     
//     println!("{}", _u.summarize());
// }

// a single struct can implement multiple traits  <T: Summary + Display> only those struct that implement both Summary and Display can be passed here

 let vj;
 vj=44;
 println!("{vj}");


fn longest<'a,'b>(a:&'a str, b: &'b str)-> &'a str  // rust compiler will say what if the b it will point to and its scope ends cause it is being borrowed so the ownership is still with them 
{
    if a.len()>b.len(){
        a
    }
    else {
        b
    }
}
let result;
let str1=String::from("vinay");
{let str2=String::from("joshidd");
  result = longest(&str1, &str2);}  // here scope changes what if result points to the str2 but its scop ends so it will be a dangling pointer even if it is small compiler gives u the error
println!("Longest string is: {}", result);
 
}

