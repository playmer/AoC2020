use std::io::{BufRead};
use std::str::FromStr;
use std::collections::HashMap;


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
    Password{ password : String::from(parts[3]), min_appearances : min, max_appearances : max, letter }
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
    for Password{ password , min_appearances : min, max_appearances : max, letter } in &passwords {
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


#[derive(Copy, Clone, PartialEq, Eq)]
enum TileType { Ground, Tree }
struct Map
{
    tiles : Vec<Vec<TileType>>, // indexed as `tiles[y][x]`
}

impl Map
{
    fn location(&self, virtual_location : (usize, usize)) -> TileType
    {
        let horizontal_map : &Vec<TileType> = &self.tiles[virtual_location.1];
        let virtual_x = virtual_location.0 % horizontal_map.len();
        horizontal_map[virtual_x]
    }
}

fn u8_to_tile_type(c : &u8) -> TileType
{
    match *c as char
    {
        '.' => TileType::Ground,
        '#' => TileType::Tree,
        _ => panic!()
    }
}

fn line_to_tile_vec(password: &String) -> Vec<TileType>
{
    password.as_bytes().iter().map(u8_to_tile_type).collect::<Vec<TileType>>()
}


fn map_from_lines(lines : &Vec<String>) -> Map
{
    Map{tiles : lines.iter().map(line_to_tile_vec).collect::<Vec<Vec<TileType>>>()}
}

fn traverse_map(map : &Map, increase_x : usize, increase_y : usize) -> usize
{
    let mut trees_hit : usize = 0;
    let mut position : (usize, usize) = (0, 0);
    loop
    {
        position.0 += increase_x;
        position.1 += increase_y;
        let tile = map.location(position);

        if tile == TileType::Tree
        {
            trees_hit += 1;
        }
        if position.1 >= (map.tiles.len() - 1) { break; }
    }

    trees_hit
}

fn day_03()
{
    println!("Day 3:");
    let map = map_from_lines(&lines_from_file("adventInput_03.txt"));


    //map.tiles.len()

    println!("    Part 1: {}", traverse_map(&map, 3, 1));

    println!("    Part 2: {}", traverse_map(&map, 1, 1)
        * traverse_map(&map, 3, 1)
        * traverse_map(&map, 5, 1)
        * traverse_map(&map, 7, 1)
        * traverse_map(&map, 1, 2));
}


struct Passport
{
    byr : String, //(Birth Year)
    iyr : String, //(Issue Year)
    eyr : String, //(Expiration Year)
    hgt : String, //(Height)
    hcl : String, //(Hair Color)
    ecl : String, //(Eye Color)
    pid : String, //(Passport ID)
    cid : String, //(Country ID)
}

struct NorthPoleCredentials
{
    byr : String, //(Birth Year)
    iyr : String, //(Issue Year)
    eyr : String, //(Expiration Year)
    hgt : String, //(Height)
    hcl : String, //(Hair Color)
    ecl : String, //(Eye Color)
    pid : String, //(Passport ID)
}

enum PurportedPassport
{
    Passport(Passport),
    NorthPoleCredentials(NorthPoleCredentials),
    Other(HashMap<String,String>)
}

fn purported_passports_from_file(filename: impl AsRef<std::path::Path>) -> Vec<PurportedPassport> {
    let file = std::fs::read_to_string(filename).expect("no such file");
    let individual_passport_datas = file.split("\r\n\r\n").collect::<Vec<&str>>();

    let mut purported_passports : Vec<PurportedPassport> = Vec::new();

    for individual_passport_data in individual_passport_datas {
        let mut purported_passport : HashMap<String, String> = HashMap::new();
        let mut purported_passport_data = individual_passport_data.split(|c| c == ' ' || c == '\n' || c == '\r').collect::<Vec<&str>>();
        purported_passport_data.retain(|&x| !x.is_empty());
        for purported_passport_datum in purported_passport_data {
            let item = purported_passport_datum.split(':').collect::<Vec<&str>>();
            purported_passport.insert(String::from(item[0]), String::from(item[1]));
        }

        purported_passports.push(PurportedPassport::Other(purported_passport));
    }

    for purported_passport in purported_passports.iter_mut() {
        match purported_passport
        {
            PurportedPassport::Other(possible) => {
                if possible.contains_key("byr")
                    && possible.contains_key("iyr")
                    && possible.contains_key("eyr")
                    && possible.contains_key("hgt")
                    && possible.contains_key("hcl")
                    && possible.contains_key("ecl")
                    && possible.contains_key("pid")
                {
                    let byr = possible.get("byr").unwrap().to_string();
                    let iyr = possible.get("iyr").unwrap().to_string();
                    let eyr = possible.get("eyr").unwrap().to_string();
                    let hgt = possible.get("hgt").unwrap().to_string();
                    let hcl = possible.get("hcl").unwrap().to_string();
                    let ecl = possible.get("ecl").unwrap().to_string();
                    let pid = possible.get("pid").unwrap().to_string();
                    if possible.contains_key("cid")
                    {
                        let cid = possible.get("cid").unwrap().to_string();
                        *purported_passport = PurportedPassport::Passport(Passport{byr, iyr, eyr, hgt, hcl, ecl, pid, cid})
                    }
                    else
                    {
                        *purported_passport = PurportedPassport::NorthPoleCredentials(NorthPoleCredentials{byr, iyr, eyr, hgt, hcl, ecl, pid})
                    }
                }
            },
            _ => panic!()
        }
    }

    purported_passports
    //let others = individual_passport_datas.iter().map(|data| data.split(|c| c == ' ' || c == '\n')).collect::<Vec<Vec<&str>>>();

    //.map(|pp| pp.split(|c| c == ' ' || c == '\n') ).collect::<Vec<Vec<String>>>()
    //let buf = std::io::BufReader::new(file);
}

//fn data_to_purported_passport() -> PurportedPassport
//{
//    PurportedPassport
//}

fn check_passport_values(
    byr : &String, //(Birth Year)
    iyr : &String, //(Issue Year)
    eyr : &String, //(Expiration Year)
    hgt : &String, //(Height)
    hcl : &String, //(Hair Color)
    ecl : &String, //(Eye Color)
    pid : &String //(Passport ID)
    ) -> bool
{
    let mut valid = true;

    //byr (Birth Year) - four digits; at least 1920 and at most 2002.
    valid = valid && (byr.len() == 4);
    let birth_year = i32::from_str(byr.as_str()).unwrap_or(0);
    valid = valid && (birth_year >= 1920) && (birth_year <= 2002);

    //iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    valid = valid && (iyr.len() == 4);
    let issue_year = i32::from_str(iyr.as_str()).unwrap_or(0);
    valid = valid && (issue_year >= 2010) && (issue_year <= 2020);

    //eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    valid = valid && (eyr.len() == 4);
    let expiration_year = i32::from_str(eyr.as_str()).unwrap_or(0);
    valid = valid && (expiration_year >= 2020) && (expiration_year <= 2030);

    //hgt (Height) - a number followed by either cm or in:
    valid = if hgt.ends_with("cm") //If cm, the number must be at least 150 and at most 193.
    {
        let number = hgt.split("cm").collect::<Vec<&str>>();
        valid = valid && number.len() == 2;
        let num_value = i32::from_str(number[0]).unwrap_or(0);

        valid = valid && ((150 <= num_value) && (num_value <= 193));
        valid
    }
    else if hgt.ends_with("in") //If in, the number must be at least 59 and at most 76.
    {
        let number = hgt.split("in").collect::<Vec<&str>>();
        valid = valid && number.len() == 2;
        let num_value = i32::from_str(number[0]).unwrap_or(0);

        valid = valid && ((59 <= num_value) && (num_value <= 76));
        valid
    }
    else
    {
        false
    };

    //hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    valid = if  hcl.chars().collect::<Vec<char>>()[0] == '#'
    {
        let alpha_range = 'a'..'f';
        let number_range = '0'..'9';
        for c in hcl[1..].chars()
        {
            valid = valid && ((alpha_range.contains(&c)) || (number_range.contains(&c)))
        }

        valid && true
    }
    else
    {
        false
    };

    //ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    valid = valid &&
        (ecl == "amb"
            || ecl == "blu"
            || ecl == "brn"
            || ecl == "gry"
            || ecl == "grn"
            || ecl == "hzl"
            || ecl == "oth");
    //pid (Passport ID) - a nine-digit number, including leading zeroes.
    valid = valid && (pid.len() == 9);
    let _expiration_year = i64::from_str(pid.as_str()).unwrap_or(0);

    valid
}

fn day_04()
{
    println!("Day 4:");
    let passports = purported_passports_from_file("adventInput_04.txt");

    let mut quote_valids_unquote : usize = 0;
    for passport in &passports {
        match passport
        {
            PurportedPassport::Passport(_) | PurportedPassport::NorthPoleCredentials(_) => quote_valids_unquote += 1,
            _ => {}
        }
    }



    println!("    Part 1: {}", quote_valids_unquote);


    quote_valids_unquote = 0;
    for passport in &passports {
        match passport
        {
            PurportedPassport::Passport(passport) => {
                if check_passport_values(&passport.byr, &passport.iyr, &passport.eyr, &passport.hgt, &passport.hcl, &passport.ecl, &passport.pid) {
                    quote_valids_unquote += 1
                }
            },
            PurportedPassport::NorthPoleCredentials(passport) => {
                if check_passport_values(&passport.byr, &passport.iyr, &passport.eyr, &passport.hgt, &passport.hcl, &passport.ecl, &passport.pid) {
                    quote_valids_unquote += 1
                }
            },
            _ => {}
        }
    }

    println!("    Part 2: {}", quote_valids_unquote);
}

fn main() {
    day_01();
    day_02();
    day_03();
    day_04();
}
