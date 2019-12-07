use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

//code from https://users.rust-lang.org/t/reading-integers-from-a-file-into-vector/17517/4
pub fn read_lines(filename: &str) -> Result<Vec<i64>, std::io::Error> {
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

pub fn read_string_to_vec(filename: &str) -> Result<Vec<usize>, std::io::Error>
{
    let file = File::open(filename)?;
    let mut br = BufReader::new(file);
    
    let mut line_str = String::new();
    let _bytes = match br.read_line(&mut line_str) {
        Ok(size) => size,
        Err(e) => return Err(e) 
    };
    let vs: Vec<&str> = line_str.split(",").collect();
    let v = vs
        .into_iter()
        .flat_map(|num| {num.trim().parse::<usize>()})
        .collect();
    Ok(v)
}