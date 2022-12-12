use std::collections::HashMap;

fn main() {
    println!("Hello, world!"); 
    
    /*
    hashmap SHA256, SHA3 -> security (encryption)
    Blake, TwoxXoncat -> hashers/hashing functions
    */
    let mut my_map = HashMap::new();
    my_map.insert(1,"");
    my_map.insert(2,"a");
    my_map.insert(3, "b");

    for(key, val) in my_map{ //write my_map.keys() for printing only keys and .values() for printing only values
        println!("{} {}", key, val);
    }
}

fn add(a:i32, b:i32) -> i32{
    a+b
}

//testing the add function

#[test]
fn add_works(){
    let res = add(2,2);
    //assertions
    assert!(res==4); // assert is a macro
    assert_eq!(res,4,"works!"); //checks if the two variables are equal
    assert_ne!(res,3); //checks if the two variables are not equal
}