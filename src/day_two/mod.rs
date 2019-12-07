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
    opcode
}

fn decode_code_slice(opcode: &mut [usize]) -> &[usize]
{
    let mut index = 0;
    loop {
        if index >= opcode.len() {
            break;
        }
        let k = opcode[index+3];
        if k >= opcode.len() {
            break;
        }
        match opcode[index] {
            1 => {
                //let k = opcode[index+3];
                opcode[k] = add_values(opcode[opcode[index+1]], opcode[opcode[index+2]]);
            },
            2 => {
                //let k = opcode[index+3];
                opcode[k] = multiply_values(opcode[opcode[index+1]], opcode[opcode[index+2]]);
            },
            99 => {break;},
            _ => {}
        }
        index += 4;
    }
    opcode
} 

fn part_two_formula(val1: usize, val2: usize) -> usize
{
    100 * val1 + val2
}

pub fn run_day_two()
{
    for i in 0..99 {
        for k in 0..99 {
            let mut opcode = get_opcode();
            opcode[1] = i;
            opcode[2] = k;
            if decode_code_slice(opcode.as_mut_slice())[0] == 19690720 {
                println!("100 * noun + verb => 100 * {} + {} = {}", i, k, part_two_formula(i, k));
                break;
            }
        }
    }
    //println!("decoded {:?}", decode_code_slice(opcode.as_mut_slice()));
    //println!("{:?}", opcode);
    
}