use crate::utils::read_lines;

pub fn solution() {
    if  let Ok(lines) = read_lines("this is \n\n the text \n\n to be read") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}
