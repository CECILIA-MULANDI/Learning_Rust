fn main() {
    let mut  v=vec![1,3,5,7];
    println!("{:?}",takes_arg(&v));
    v.push(12);
    println!("{:?}",v);
   
    add_two(3);

}

fn takes_arg(val:&Vec<i32>)->bool{
    
    loop{
        if val[0]==1{
            return true
        }
        else{
            return false
        }    
    }
}

fn add_two(num:i8){
   

    let sum = num+2;
    println!("the sum is {}",sum);

}