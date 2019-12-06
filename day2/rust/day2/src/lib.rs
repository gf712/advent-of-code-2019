fn process_operations(vec: &mut Vec<usize>) {
    let mut i: usize = 0;
    while i < vec.len() {
        let value = match vec[i] {
            1 => vec[vec[i + 1]] + vec[vec[i + 2]],
            2 => vec[vec[i + 1]] * vec[vec[i + 2]],
            _ => return,
        };

        let pos = vec[i + 3];
        vec[pos] = value;

        i += 4;
    }
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

pub fn add_launch_code(string: String, noun: String, verb: String) -> String {
    let mut result_vec = Vec::<String>::new();
    let mut internal_string = string;

    remove_whitespace(&mut internal_string);

    let mut iter = internal_string.split(",");

    result_vec.push(iter.next().unwrap().to_string());
    result_vec.push(noun);
    result_vec.push(verb);

    iter.next();
    iter.next();

    for el in iter {
        result_vec.push(el.to_string());
    }

    return result_vec.join(",");
}

pub fn parse(string: String) -> String {
    let mut result_vec = Vec::<usize>::new();
    let mut internal_string = string;

    remove_whitespace(&mut internal_string);

    // for convenience translate string to Vec<usize>
    for el in internal_string.split(",") {
        let value = el.parse::<usize>().expect("Unable to parse value as int");
        result_vec.push(value);
    }

    process_operations(&mut result_vec);

    let mut string_result_vec = Vec::<String>::with_capacity(result_vec.len());

    // convert Vec<usize> to string and then join (more convenient than adding the ",")
    for el in result_vec.iter() {
        string_result_vec.push(el.to_string());
    }

    return string_result_vec.join(",");
}
