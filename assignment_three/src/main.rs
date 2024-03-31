#[derive(Debug)]
struct Car{
    mpg:i8,
    color:String,
    top_speed:i32

}

impl Car{
    fn set_mpg(&mut self,mpg:i8){
        self.mpg=mpg
       
    }
    fn set_color(&mut self,color:String){
        self.color=color

    }
    fn set_top_speed(&mut self,speed:i32){
        self.top_speed=speed
       

    }
}

fn main() {
    let mut car1=Car{mpg:2,color:String::from("yellow"),top_speed:1};
    println!("{:?}",car1);
   car1.set_mpg(4);
   car1.set_color(String::from("red"));
   car1.set_top_speed(4);
   println!("{:?}",car1);
}