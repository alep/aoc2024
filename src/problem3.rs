use std::fs;

pub fn main() {
    let filepath = "./inputs/input3.txt";
    let contents = fs::read_to_string(filepath).expect("there should of been a file.");
    println!("{:?}", parse(&contents));
}

pub fn parse(contents: &str) -> usize {
    let contents: Vec<char> = contents.chars().collect();
    let mut current = 0;
    let mut sum = 0;
    let mut enable_mul = true;
    'main: loop {
        if current >= contents.len() {
            break 'main;
        }

        let c = contents[current];
        if c == 'm' {
            let expected = "ul(";

            if current + expected.len() > contents.len() {
                break 'main;
            } else if current + expected.len() < contents.len() {
                if &contents[current..current + expected.len() + 1] == &['m', 'u', 'l', '('] {
                    current += expected.len() + 1;

                    let right: usize;
                    let left: usize;

                    // parse digits
                    match parse_digits(&contents, &mut current) {
                        Ok(num) => right = num,
                        Err(()) => continue 'main,
                    }
                    // expect a comma
                    if contents[current] != ',' {
                        continue 'main;
                    }

                    // expected comma
                    current += 1;

                    // parse next argument
                    match parse_digits(&contents, &mut current) {
                        Ok(num) => left = num,
                        Err(()) => continue 'main,
                    }
                    // parse closing ')'
                    if contents[current] != ')' {
                        continue 'main;
                    }
                    if enable_mul {
                        sum += right * left;
                    }
                } else {
                    current += 1;
                    continue 'main;
                }
            } else {
                current += 1;
                continue 'main;
            }
        } else if c == 'd' {
            if current + "on't()".len() > contents.len() {
                break 'main;
            } else if current + "o()".len() > contents.len() {
                break 'main;
            } else if &contents[current..current + "on't()".len() + 1]
                == &['d', 'o', 'n', '\'', 't', '(', ')']
            {
                enable_mul = false;
                current += "on't()".len() + 1;
            } else if &contents[current..current + "o()".len() + 1] == &['d', 'o', '(', ')'] {
                enable_mul = true;
                current += "o()".len() + 1;
            } else {
                current += 1;
            }
        } else {
            current += 1;
            continue;
        }
    }
    return sum;
}

pub fn parse_digits(contents: &Vec<char>, current: &mut usize) -> Result<usize, ()> {
    let mut num = String::new();
    loop {
        if *current > contents.len() {
            return Err(());
        }

        if contents[*current].is_ascii_digit() {
            num.push(contents[*current]);
            *current += 1;
        } else {
            break;
        }
    }
    Ok(num
        .parse::<usize>()
        .expect("I just parsed a number what the f?"))
}

#[cfg(test)]
mod test_this_shit {
    use crate::parse;

    #[test]
    fn test_parse() {
        assert_eq!(parse("asdfasdfasmul(1,1)m[123]mul(1,2mul(1,1)"), 2);
        assert_eq!(
            parse("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
        assert_eq!(
            parse("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
