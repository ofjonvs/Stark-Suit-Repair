#[derive(Debug)]
#[derive(PartialEq)]
pub enum Command
{
    Power(bool,i32),    // [Increase/Decrease] power by [number].
    Missiles(bool,i32), // [Increase/Decrease] missiles by [number].
    Shield(bool),       // Turn [On/Off] the shield.
    Try,                // Try calling pepper.
    Invalid             // [anything else]
}


/**
    Adds functionality to Command enums
    Commands can be converted to strings with the as_str method
    
    Command     |     String format
    ---------------------------------------------------------
    Power       |  /Power (increased|decreased) by [0-9]+%/
    Missiles    |  /Missiles (increased|decreased) by [0-9]+/
    Shield      |  /Shield turned (on|off)/
    Try         |  /Call attempt failed/
    Invalid     |  /Not a command/
**/
impl Command {
    pub fn as_str (&self) -> String {
        let mut s = String::new();
        
        match self{
            Command::Power(in_or_dec, power) =>{
                s.push_str("Power");
                if *in_or_dec{s.push_str(" increased")}
                else{s.push_str(" decreased")}
                s.push_str(&format!(" by {}%", power));
            }
            Command::Missiles(in_or_dec, missiles) =>{
                s.push_str("Missiles");
                if *in_or_dec{s.push_str(" increased")}
                else{s.push_str(" decreased")}
                s.push_str(&format!(" by {}", missiles));
            },
            Command::Shield(on_or_off) =>{
                s.push_str("Shield turned");
                if *on_or_off{s.push_str(" on")}
                else{s.push_str(" off")}
            },
            Command::Try =>{
                s.push_str("Call attempt failed");
            },
            _ => return "Not a command".to_string()
        }

        return s
    }
}

/**
    Complete this method that converts a string to a command 
    We list the format of the input strings below

    Command     |     String format
    ---------------------------------------------
    Power       |  /power (inc|dec) [0-9]+/
    Missiles    |  /(fire|add) [0-9]+ missiles/
    Shield      |  /shield (on|off)/
    Try         |  /try calling Miss Potts/
    Invalid     |  Anything else
**/
pub fn to_command(s: &str) -> Command {
    if s == "try calling Miss Potts"{return Command::Try}
    let split_str: Vec<&str> = s.split(" ").collect();
    if split_str[0] == "power"{
        if split_str.len() != 3{return Command::Invalid}
        if split_str[1] == "inc"{
        let num = split_str[2].parse::<i32>();
        match num {
            Ok(val) => return Command::Power(true, val),
            _ => return Command::Invalid,
            }
        }
        else if split_str[1] == "dec"{
            let num = split_str[2].parse::<i32>();
            match num {
                Ok(val) => return Command::Power(false, val),
                _ => return Command::Invalid,
                }
            }
        else{return Command::Invalid}      
    }
    else if split_str[0] == "fire"{
        if split_str.len() != 3{return Command::Invalid}
        if split_str[2] != "missiles"{
            return Command::Invalid
        }
        let num = split_str[1].parse::<i32>();
        match num {
            Ok(val) => return Command::Missiles(false, val),
            _ => return Command::Invalid,
        }
    }
    else if split_str[0] == "add"{
        if split_str.len() != 3{return Command::Invalid}
        if split_str[2] != "missiles"{
            return Command::Invalid
        }
        let num = split_str[1].parse::<i32>();
        match num {
            Ok(val) => return Command::Missiles(true, val),
            _ => return Command::Invalid,
        }
    }
    else if split_str[0] == "shield"{
        if split_str.len() != 2{return Command::Invalid}
        if split_str[1] == "on"{return Command::Shield(true)}
        if split_str[1] == "off"{return Command::Shield(false)}
    }

    Command::Invalid
}


fn main() {
    assert_eq!("Call attempt failed", Command::Try.as_str());
    assert_eq!("Not a command", Command::Invalid.as_str());

    assert_eq!(Command::Try, to_command("try calling Miss Potts"));
    assert_eq!(Command::Invalid, to_command("jarvis!"));
}