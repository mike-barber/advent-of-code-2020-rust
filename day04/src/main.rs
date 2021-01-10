use regex::Regex;
use std::error::Error;

#[derive(Debug)]
struct Field {
    name: String,
    value: String,
}
#[derive(Debug)]
struct Passport {
    fields: Vec<Field>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string("input.txt")?;
    let passports_data: Vec<_> = contents.split("\n\n").collect();

    let mut passports = Vec::new();
    let rx = Regex::new("([a-z]+):(\\S+)")?;
    for p in passports_data {
        let fields = rx
            .captures_iter(p)
            .map(|c| Field {
                name: c[1].to_string(),
                value: c[2].to_string(),
            })
            .collect();
        passports.push(Passport { fields });
    }

    let required_fields = [
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
        //"cid",  // not required
    ];

    // part 1
    {
        let mut valid_count = 0;
        for p in &passports {
            let is_valid = required_fields
                .iter()
                .all(|&expected| p.fields.iter().any(|f| f.name == expected));
            if is_valid {
                valid_count += 1;
            }
        }

        println!("Part1 -> valid passports = {}", valid_count);
    }

    // part 2 -- validation
    {
        let mut valid_count = 0;
        for p in &passports {
            // check we have all the expected fields
            let has_all_fields = required_fields
                .iter()
                .all(|&expected| p.fields.iter().any(|f| f.name == expected));

            // check all the fields are valid
            let fields_are_valid = p.fields.iter().all(|f| field_validate(f));

            if has_all_fields && fields_are_valid {
                valid_count += 1;
            }
        }

        println!("Part2 -> valid passports = {}", valid_count);
    }

    Ok(())
}

fn field_validate(field: &Field) -> bool {
    match field.name.as_str() {
        "byr" => year_validate(&field.value, 1920, 2002),
        "iyr" => year_validate(&field.value, 2010, 2020),
        "eyr" => year_validate(&field.value, 2020, 2030),
        "hgt" => height_validate(&field.value),
        "hcl" => hair_colour_validate(&field.value),
        "ecl" => eye_colour_validate(&field.value),
        "pid" => passport_number_valid(&field.value),
        "cid" => true, // always accepted
        _ => {
            // debugging
            println!("invalid field: {:?}", field);
            false
        }
    }
}

fn year_validate(value: &str, min: i32, max: i32) -> bool {
    if let Ok(num) = value.parse::<i32>() {
        num >= min && num <= max
    } else {
        false
    }
}

fn height_validate(value: &str) -> bool {
    let rx = Regex::new("(\\d+)(cm|in)").expect("regex");
    if let Some(cap) = rx.captures(value) {
        if let Ok(num) = cap[1].parse::<i32>() {
            let unit = &cap[2];
            match unit {
                "cm" => num >= 150 && num <= 193,
                "in" => num >= 59 && num <= 76,
                _ => false,
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn hair_colour_validate(value: &str) -> bool {
    if value.len() != 7 {
        return false;
    }
    if value.chars().nth(0).unwrap() != '#' {
        return false;
    }
    value.chars().skip(1).all(|c| match c {
        '0'..='9' => true,
        'a'..='f' => true,
        _ => false,
    })
}

fn eye_colour_validate(value: &str) -> bool {
    match value {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn passport_number_valid(value: &str) -> bool {
    value.chars().count() == 9
        && value.chars().all(|c| match c {
            '0'..='9' => true,
            _ => false,
        })
}
