// Create a vector with the values 1, 3, 5, 7, and 9. 
// Then use an iterator and a closure to multiply all of the values by 10 
// and store the result in another vector. Print out the vector to confirm your results.




fn main() {
    let  vec:Vec<u32>=vec![1,3,5,7,9];
    let mut new_vec:Vec<u32>=Vec::new();
    let mut ans=|x| new_vec.push(x*10);
    for &i in vec.iter(){
        ans(i);

    }
    
    println!("{:?}",new_vec);
}
