use crate::read_file;

const FILENAME: &str = "data/module_mass.txt";

fn get_mass_vec() -> Vec<i64>
{
    let mass_vec = read_file::read_lines(FILENAME);
    match mass_vec {
        Ok(v) => v,
        Err(e) => { println!("error occured, {:?}", e); Vec::new()}
    }
}

fn total_fuel_req(mass_vec:Vec<i64>) -> i64
{
    mass_vec.iter().fold(0, |acc, cur| {
        acc + fuel_for_fuel(*cur)
    })
}

fn fuel_for_fuel(mass:i64) -> i64
{
    let fuel:i64 = mass/3-2;
    if fuel <= 0 {
        0
    } else {
        fuel + fuel_for_fuel(fuel)
    }
}

pub fn run_day_one()
{
    let mass_vec = get_mass_vec();
    println!("{:?}", total_fuel_req(mass_vec));
}