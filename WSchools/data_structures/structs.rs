// structure : custom data structure group data together.


struct person { 
    name:String, // fields
    age: u32,
    can_vote:bool,
}



fn main(){
    let user = person{
        name: String::from("Ujjwal Dikshit"),
        age: 32,
        can_vote: true,
    };
    //To change a value inside a struct, you must make the struct object mutable by using mut:

    let mut user = person{
        name:String::from("Ujjwal Dikshit"),
        age:35,
        can_vote:false,
    };

    // mutate | chage value in object
    user.age= 232;
    println!("Name: {}",user.name);
    println!("Age: {}",user.age);
    println!("can vote? {}",user.can_vote);
}