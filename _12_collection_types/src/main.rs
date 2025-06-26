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
}
