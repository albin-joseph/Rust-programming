use std::{collections, string};

fn main() {
    println!("Collection Types");

    //Vector
    let mut _v: Vec<i32> = Vec::new();
    let _the_vec:Vec<i32> = vec![1,2,3,4];

    let mut _v:Vec<i32> = vec![1,2,3,4];
    _v.push(5);
    _v.push(6);
    _v.push(7);
    _v.push(8);
    _v.push(9);

      println!("{:?}", _v);

      let third: &i32 = &_v[2]; 
      println!("The third element in _v:{third}");

     let third = _v.get(2);

     match third {
         Some(third) => println!("The third element for GET method is {third}"),
         None => println!("There is no third lement")
     }


     //UTF8
     let s = "hello".to_string();
     let s: String = String::from("hello");
     let mut s: String = String:: from("foo");
     s.push_str("bar");
     s.push('!');
     
     println!("the value of s is equal to {}", s);

     let s1  = "Hello".to_string();
     let s2 = " World".to_string();

     let s3 = s1 + &s2;

     println!("s2: {}", s3);

     //HashMap

     use std::collections::HashMap;
     let mut scores: HashMap<String, i32> = HashMap::new();
     scores.insert(String::from("Black"), 10);
     scores.insert(String::from("White"), 11);

     let team_name = String::from("Black");
     let score = scores.get(&team_name).copied().unwrap_or(0);

     for(key, value) in &scores {
        println!("{key}: {value}")
     }




}
