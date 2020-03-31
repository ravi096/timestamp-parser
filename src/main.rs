fn main() {
    // Make sure to have a space at the end (as a delim)
    let datestr = "2019-08-22 03:47:00,432 - 140088981829376 ".to_string();
    println!("{}", datestr);
    
    let mut st_index = 0;
    let mut _prev_type = 'D'; // 'D', 'I', 'C'
    //let mut format_str = String::new();
    
    //let mut is_end = false;
    
    for (i, c) in datestr.chars().enumerate() {
        if i == datestr.len() {
            println!("Reached the end of the string");
            //_is_end = true;
        }
        
        match c {
            '0'..='9' => {
                // println!("{:?} -- INT", c);
                _prev_type = 'I';
            },
            'a'..='z' | 'A' ..= 'Z' => {
                // println!("{:?} -- CHAR", c);
                _prev_type = 'C';
            },
            _ => {
                
                match _prev_type {
                    'C' => {
                        println!("{} -- CHAR", &datestr[st_index..i]);
                    },
                    'I' => {
                        let len = i - st_index;
                        println!("{} -- INT of size {}", &datestr[st_index..i], len);
                        
                        match len {
                            4 => {
                                println!("Year in YYYY format");
                            },
                            3 => {
                                println!("Milliseconds in SSS format");
                            },
                            2 => {
                                println!("Could be YY, MM, DD, HH, MM, SS");
                                /*
                                match datestr[st_index] {
                                    4, 5, 6 => {
                                        println!("Cannot be DD, MM")  
                                    },
                                    3 => {
                                        
                                    },
                                    2 => {
                                        
                                    },
                                    1 => {
                                        
                                    },
                                    0 => {
                                        
                                    },
                                    _ => {
                                        
                                    }
                                }
                                */
                            }
                            _ => {
                                println!("Unknown");
                            }
                                
                            
                        }
                    },
                    _ => {}

                }
                //println!("{:?} -- DELIM", c);
                _prev_type = 'D';
                st_index = i + 1;
            }
        }
    }
}
