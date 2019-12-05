pub fn compute_fuel_requirement(mass: u32) -> u32 {
    if mass <= 8 {
        return 0;
    }
    return (mass / 3) - 2;
}

pub fn compute_complete_fuel_requirement(mass: u32) -> u32 {
    let mut result = compute_fuel_requirement(mass);
    let mut remainder = result;
    while remainder > 0 {
        remainder = compute_fuel_requirement(remainder);
        result += remainder;
    }
    return result;
}
