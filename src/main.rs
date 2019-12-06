use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

const FILENAME: &str = "data/module_mass.txt";

//code from https://users.rust-lang.org/t/reading-integers-from-a-file-into-vector/17517/4
fn read_lines(filename: &str) -> Result<Vec<i64>, std::io::Error> {
    let file = File::open(filename)?; // open file by given path
    // wrap file into generic buffered reader, it will use 4 KB buffer internally
    // to reduce number of syscalls, thus improving performance
    let br = BufReader::new(file);
    // create an empty vector, type of the stored elements will be inferred
    let mut v = Vec::new();
    // br.lines() creates an iterator over lines in the reader
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
    for line in br.lines() {
        // IO operations generally can return error, we check if got
        // an error,in which case we return this error as the function result
        let line = line?;
        let n = line   
            .trim() // trim "whitespaces"
            .parse() // call `str::parse::<i64>(&self)` method on the trimmed line, which parses integer
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; // parse() can return error (e.g. for string "abc"), here if we got it, we convert it to `std::io::Error` type and return it as function result
        v.push(n); // push acquired integer to the vector
    }
    Ok(v) // everything is Ok, return vector
}

fn get_mass_vec() -> Vec<i64>
{
    let mass_vec = read_lines(FILENAME);
    match mass_vec {
        Ok(v) => v,
        Err(e) => { println!("error occured, {:?}", e); Vec::new()}
    }
}

fn total_fuel_req(mass_vec:Vec<i64>) -> i64
{
    mass_vec.iter().fold(0, |acc, cur| { acc + (cur/3-2) })
}

fn main() {
    let mass_vec = get_mass_vec();
    println!("{:?}", total_fuel_req(mass_vec));
}
