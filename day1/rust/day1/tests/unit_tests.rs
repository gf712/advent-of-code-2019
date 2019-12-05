#[cfg(test)]
mod tests {
    #[test]
    fn day1_part1_tests() {
        assert_eq!(day1::compute_fuel_requirement(12), 2);
        assert_eq!(day1::compute_fuel_requirement(14), 2);
        assert_eq!(day1::compute_fuel_requirement(1969), 654);
        assert_eq!(day1::compute_fuel_requirement(100756), 33583);
    }
    #[test]
    fn day1_part2_tests() {
        assert_eq!(day1::compute_complete_fuel_requirement(12), 2);
        assert_eq!(day1::compute_complete_fuel_requirement(1969), 966);
        assert_eq!(day1::compute_complete_fuel_requirement(100756), 50346);
    }
}
