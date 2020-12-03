use std::io::BufRead;
use std::str::FromStr;


fn lines_from_file(filename: impl AsRef<std::path::Path>) -> Vec<String> {
    let file = std::fs::File::open(filename).expect("no such file");
    let buf = std::io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn day_01()
{
    let inputs = lines_from_file("adventInput_01.txt");

    let expense_report_entries = inputs.iter().map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut value_found1 = 0;

    for (i, expense_report_entry1) in expense_report_entries.iter().enumerate() {
        for (j, expense_report_entry2) in expense_report_entries.iter().enumerate() {
            if i == j  { continue; }
            if 2020 == (expense_report_entry1 + expense_report_entry2)
            {
                value_found1 = expense_report_entry1 * expense_report_entry2;
                break;
            }
        }
    };

    println!("Day 1:");
    println!("    Part 1: {}", value_found1);


    let mut value_found1 = 0;

    for (i, expense_report_entry1) in expense_report_entries.iter().enumerate() {
        for (j, expense_report_entry2) in expense_report_entries.iter().enumerate() {
            for (k, expense_report_entry3) in expense_report_entries.iter().enumerate() {
                if i == j && i == k { continue; }
                if 2020 == (expense_report_entry1 + expense_report_entry2 + expense_report_entry3)
                {
                    value_found1 = expense_report_entry1 * expense_report_entry2 * expense_report_entry3;
                    break;
                }
            }
        }
    };

    println!("    Part 2: {}", value_found1);
}

#[derive(Debug, PartialEq)]
struct Password
{
    password : String,

    // policy info
    min_appearances : i32,
    max_appearances : i32,
    letter : char,
}


fn string_to_password(password: &String) -> Password {
    let mut parts: Vec<&str> = password.split(|c| c == ' ' || c == ':' || c == '-').collect::<Vec<&str>>();
    parts.retain(|&x| !x.is_empty());

    let min: i32 = i32::from_str(parts[0]).unwrap();
    let max: i32 = i32::from_str(parts[1]).unwrap();
    let letter: char = char::from_str(parts[2]).unwrap();
    Password{ password : String::from(parts[3]), min_appearances : min, max_appearances : max, letter : letter}
}

fn day_02()
{
    println!("Day 2:");

    let inputs = lines_from_file("adventInput_02.txt");

    let passwords = inputs.iter().map(string_to_password).collect::<Vec<Password>>();

    let mut matching_passwords : usize = 0;
    for Password{ password , min_appearances : min, max_appearances : max, letter } in &passwords {
        let count  = password.matches(*letter).count();

        if count >= *min as usize && count <= *max as usize
        {
            matching_passwords += 1;
        }
    }

    println!("    Part 1: {}", matching_passwords);


    matching_passwords = 0;
    for Password{ password , min_appearances : min, max_appearances : max, letter : letter } in &passwords {
        let toboggan_min : usize = *min as usize - 1;
        let toboggan_max : usize = *max as usize - 1;
        let min_char = password.as_bytes()[toboggan_min] as char;
        let max_char =password.as_bytes()[toboggan_max] as char;

        if min_char == *letter && max_char != *letter
            || min_char != *letter && max_char == *letter
        {
            matching_passwords += 1;
        }
    }

    println!("    Part 2: {}", matching_passwords);
}



fn day_03()
{
    println!("Day 3:");
    let lines = lines_from_file("adventInput_03.txt");

    println!("    Part 1: ");
    println!("    Part 2: ");
}


fn main() {
    day_01();
    day_02();
}
