fn main() {
//    let val1=5;
//    let val2=2;
//    let ans=val1%val2;
//    println!("the modulo of 5 % 2 is {}",ans);
    // let mut v = vec![2,4,6,8,10];
    // println!("the current vector is {:?}",v);
    // v.pop();
    // println!("the vector after doing the pop operation is {:?}",v);
    // v.push(12);
    // println!("the vector after doing the push operation is {:?}",v);
    println!("{}",concat_string("Hello"));
    control_flow(29);
   

}

fn concat_string(s:&str)->String{
    let final_string=format!("{},{}",s,"World");
    final_string
    
}

fn control_flow(a:i32){
    if a == 1{
        println!("the value {}, is one",a);
    }else if a>50{
        println!("the value  {}, is  greater than 50",a);


    }
    else if a<25{
        println!("the value {}, is less than 25",a);


    }
    else{
        println!("the value {}, is greater than 25 and less than 50",a);
    }

}