#[cfg(test)]
mod tests {
    #[test]
    fn day1_part1_1_tests() {
        let test_string = String::from("1,0,0,0,99");
        assert_eq!(day2::parse(test_string), "2,0,0,0,99");
    }
    #[test]
    fn day1_part1_2_tests() {
        let test_string = String::from("2,3,0,3,99");
        assert_eq!(day2::parse(test_string), "2,3,0,6,99");
    }
    #[test]
    fn day1_part1_3_tests() {
        let test_string = String::from("2,4,4,5,99,0");
        assert_eq!(day2::parse(test_string), "2,4,4,5,99,9801");
    }
    #[test]
    fn day1_part1_4_tests() {
        let test_string = String::from("1,1,1,4,99,5,6,0,99");
        assert_eq!(day2::parse(test_string), "30,1,1,4,2,5,6,0,99");
    }
    #[test]
    fn day1_part1_5_tests() {
        let test_string = String::from("1,9,10,3,2,3,11,0,99,30,40,50");
        assert_eq!(
            day2::parse(test_string),
            "3500,9,10,70,2,3,11,0,99,30,40,50"
        );
    }
}
