#[derive(Debug)]
enum Shape{
    Triangle,
    Square,
    Pentagon, 
    Octagon
}
impl Shape{
    fn return_corners(&self)->i8{
        match self{
            Shape::Triangle=>3,
            Shape::Square=>4,
            Shape::Pentagon=>5,

            _=>8,
           
            

        }
    }
}

fn main() {
 let mut shapes=Shape::Octagon;
 println!("{:?}",shapes.return_corners());
}
