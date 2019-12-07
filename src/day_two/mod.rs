use crate::read_file;

const FILENAME: &str = "data/opcode.txt";

fn get_opcode() -> Vec<usize>
{
    let opcode = read_file::read_string_to_vec(FILENAME);
    match opcode {
        Ok(code) => code,
        Err(e) => { println!("error occured, {:?}", e); Vec::new()}
    }
}

fn add_values(num1: usize, num2: usize) -> usize
{
    num1 + num2
}

fn multiply_values(num1: usize, num2: usize) -> usize
{
    num1 * num2
}

fn decode_code(mut opcode:Vec<usize>) -> Vec<usize>
{
    println!("{:?}", opcode);
    let mut index = 0;
    loop {
        if index >= opcode.len() {
            break;
        }
        match opcode[index] {
            1 => {
                let k = opcode[index+3];
                opcode[k] = add_values(opcode[opcode[index+1]], opcode[opcode[index+2]]);
            },
            2 => {
                let k = opcode[index+3];
                opcode[k] = multiply_values(opcode[opcode[index+1]], opcode[opcode[index+2]]);
            },
            99 => {break;},
            _ => {}
        }
        index += 4;
    }
    println!("{:?}", opcode);
    opcode
}

pub fn run_day_two()
{
    let opcode = get_opcode();
    decode_code(opcode);
}