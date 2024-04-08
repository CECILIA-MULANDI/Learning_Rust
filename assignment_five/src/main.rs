// Modify the solution to the Section 4 assignment
//  by creating a Trait that has the set_mpg, set_color, and set_top_speed methods.
//  Then create a Motorcycle struct with the same fields as the Car struct:
//  mpg, color, and top_speed. Now implement your Trait on both the Car and Motorcycle struct.
//  Print out the results to confirm a working solution.

// Create a simple print function that uses Generic T. 
// This Generic T will need to implement 
// std::fmt::Debug depending on the values you pass in. 
// Our function takes one parameter of type T. 
// Our function will then print out the value that is passed in.

fn printing<T:std::fmt::Debug>(data:T){
    let new_data=data;
    println!("{:?}",new_data)

}


#[derive(Debug)]
struct Car{
    mpg:u32,
    color:String,
    top_speed:u32,

}
#[derive(Debug)]
struct Motorcycle{
    mpg:u32,
    color:String,
    top_speed:u32,

}
trait CarAlterations{
    fn set_mpg(&mut self,mpg:u32);
    fn set_color(&mut self,color:String);
    fn set_top_speed(&mut self,top_speed:u32);
}
impl CarAlterations for Car{
    fn set_mpg(&mut self,mpg:u32){
        self.mpg=mpg
    }
    fn set_color(&mut self,color:String){
        self.color=color
    }
    fn set_top_speed(&mut self,top_speed:u32){
        self.top_speed=top_speed
    }

}
impl CarAlterations for Motorcycle{
    fn set_mpg(&mut self,mpg:u32){
        self.mpg=mpg
    }
    fn set_color(&mut self,color:String){
        self.color=color
    }
    fn set_top_speed(&mut self,top_speed:u32){
        self.top_speed=top_speed
    }

}
fn main() {
    let mut car1=Car{mpg:0,color:String::from("Yellow"),top_speed:200};
    println!("The car had this features at first :{:?}",car1);
    let mut motorcycle1=Motorcycle{mpg:10,color:String::from("Silver"),top_speed:400};
    println!("The motorcycle haGreend this features at first :{:?}",motorcycle1);
    car1.set_mpg(2);
    car1.set_color(String::from("Green"));
    car1.set_top_speed(900);
    println!("After the changes it had this features{:?}",car1);
    motorcycle1.set_mpg(20);
    motorcycle1.set_color(String::from("White"));
    motorcycle1.set_top_speed(1900);
    println!("After the changes it had this features{:?}",motorcycle1);
    
        printing(String::from("colors"));
 
   
    }
 