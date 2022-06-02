use std::io;

#[derive(Debug)]
struct Information {
    username: String,
    amount: u32,
    address: String,
    email: String,
}

fn main () {

    loop {
    let mut unparsed_amount = String::new();

    let mut username = String::new();
    let mut unparsed_amount = String::new();
    let mut address = String::new();
    let mut email = String::new();
    let mut y_or_n = String::new();

    println!("Please enter your RBX username.");

    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read input");
    
    println!("Please enter your desired amount of RBX.");

    io::stdin()
        .read_line(&mut unparsed_amount)
        .expect("Failed to read input");

        let parsed_amount: u32 = to_u32(unparsed_amount);

    println!("Please enter your address. Example: 1234 Weed Lane, Nashville, Tennesee, United States of America");

    io::stdin()
        .read_line(&mut address)
        .expect("Failed to read input");

    println!("Please enter your email. Example: example@gmail.com");

    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read input");
    
    let info1 = build_info(username, parsed_amount, email, address);

    println!("You have entered this information: {:#?}", info1);
    println!("Is this correct? (Y/N)");
    
    io::stdin() 
        .read_line(&mut y_or_n)
        .expect("Failed to read input");

    let parsed_y_or_n = y_or_n.chars().next().unwrap();

    match parsed_y_or_n {
        Y => {
            println!("The RBX has now been INJECTED.");
            break
        },
        y => {
            println!("The RBX has now been INJECTED.");
            break
        },
        n => {
            println!("your computer inejct with virus destroy now")
        },
    }
    
}
}
    
fn to_u32 (amt: String) -> u32 {
    let int: u32 = match amt.trim().parse() {
        Ok(parsed) => parsed,
        Err(error) => panic!("virus has been detected on your computer remove and destroy hard drive now: error: {:?}", error),
    };

    return int;
}

fn build_info (username: String, amount: u32, email: String, address: String) -> Information {
    Information {
        username,
        amount,
        email,
        address,
    }
}

