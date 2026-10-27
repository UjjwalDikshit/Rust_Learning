// constant variable type must be defined with a type
fn main(){
    const BIRTHYEAR: i32 = 1980;
    const MINUTES_PER_HOUR : i32 = 60;

    const BIRTHYEAR: i32 = 1980; // Ok
    const BIRTHYEAR = 1980; // Error: missing type


}

// Feature	       Constant (const)          Variable (let)
// Can change?	       No	                 Yes, if mut is used
// Type required?	   Yes	                 No (optional)

