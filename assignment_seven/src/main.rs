// Create a variable on the stack and a variable on the heap. 
// Multiply their values and print out the results.


// Create a variable that holds a String. 
// Then create a reference counting smart pointer that contains the String. 
// Print out how many references the smart pointer has. 
// Now inside the code block create another reference counting 
// smart pointer that points to our first smart pointer. 
// Print out how many references each smart pointer has.
use std::rc::Rc;
fn main() {
    let str1=String::from("Cecilia");
    {
        let str1_ref=Rc::new(str1);
        println!("{}",Rc::strong_count(&str1_ref));
        {
            let str1_ref2=Rc::clone(&str1_ref);
            println!("{}",Rc::strong_count(&str1_ref));
            println!("{}",Rc::strong_count(&str1_ref2));
        }
    }
    
  


    let a = 4;
    let b = Box::new(6);
  
    let ans = a* *b;

    println!("the product is: {}",ans);
}
