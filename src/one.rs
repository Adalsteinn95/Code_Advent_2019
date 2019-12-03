pub fn one(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for number in numbers.iter() {
        sum += number / 3 - 2;
    }

    return sum;
}

pub fn onepart2(masses: &[i32]) -> i32 {
    masses
        .iter()
        .fold(0, |sum, mass| sum + mass_to_fuel_2(*mass))
}

fn mass_to_fuel_2(mass: i32) -> i32 {
    let mut sum = 0;
    let mut current_mass = mass;
    loop {
        let fuel = current_mass / 3 - 2;
        if fuel <= 0 {
            break;
        }
        current_mass = fuel;
        sum += fuel;
    }
    sum
}
