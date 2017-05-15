use std::io;

pub fn user_input(msg: &str) -> String {
	println!("{}", msg);
	let mut input = String::new();		
	match io::stdin().read_line(&mut input) {	    
		Ok(..) => return input,
	    Err(error) => {
			panic!("error: {}", error);			
		}
	}
}

pub fn input2f64(input: &str, result: &mut f64) -> bool {		
    match input.trim().parse::<f64>() {
        Ok(num) => {
			*result = num;
			return true;
		}
        Err(..) => return false,
    };	
}