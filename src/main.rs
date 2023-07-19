use chrono::{DateTime, Duration, Local, NaiveDate};
fn main() {
    // median
    let mut v = vec![1, 2, 3, 4, 5];
    let mut median_value = median(&mut v);
    println!("The median of {:?} is {}", v, median_value.unwrap());
    v.push(6);
    median_value = median(&mut v);
    println!("The median of {:?} is {}", v, median_value.unwrap());

    // remove duplicates
    let mut v = vec![1, 2, 3, 4, 5, 5, 5, 6, 7, 8];
    println!("Before removing duplicates: {:?}", v);
    remove_duplicates(&mut v);
    println!("After removing duplicates: {:?}", v);

    // remove duplicates generic
    let mut v2 = vec![1, 2, 3, 4, 5, 5, 5, 6, 7, 8];
    println!("Before removing duplicates (generic): {:?}", v2);
    remove_duplicates_generic(&mut v2);
    println!("After removing duplicates: {:?}", v2);

    // print any string
    let string_slice: &str = "Hello, string slice!";
    let string_object: String = String::from("Hello, world!");
    print_any_string(string_slice);
    print_any_string(string_object);

    // case sensitive sort (default)
    let mut v = vec![
        "Hello",
        "hello",
        "World",
        "world",
        "Rust",
        "rust",
        "Programming",
        "programming",
    ];
    println!("Before case sensitive sort: {:?}", v);
    case_sensitive_sort(&mut v);
    println!("After case sensitive sort: {:?}", v);

    // case insensitive sort
    let mut v2 = vec![
        "Hello",
        "hello",
        "World",
        "world",
        "Rust",
        "rust",
        "Programming",
        "programming",
    ];
    println!("Before case insensitive sort: {:?}", v2);
    case_insensitive_sort(&mut v2);
    println!("After case insensitive sort: {:?}", v2);

    // deadline
    let future_event = ImportantEvent {
        name: String::from("My important event"),
        timestamp: Local::now() + Duration::days(10),
    };
    println!("Is the event passed? {}", future_event.is_passed());

    let past_event = ImportantEvent {
        name: String::from("My important event"),
        timestamp: Local::now() - Duration::days(10),
    };
    println!("Is the event passed? {}", past_event.is_passed());

    // sum a vector of optional integers
    let v = vec![Some(1), Some(2), Some(3), Some(4), Some(5)];
    println!("The sum of {:?} is {}", v, sum(&v));
    let v = vec![Some(1), Some(2), None, Some(4), Some(5)];
    println!("The sum of {:?} is {}", v, sum(&v));

    // weeks between
    let date1 = "2020-01-01";
    let date2 = "2020-01-31";
    println!(
        "There are {} weeks between {} and {}",
        weeks_between(date1, date2),
        date1,
        date2
    );

    // file exists
    let path = "src/main.rs";
    println!("Does the file {} exist? {}", path, file_exists(path));

    // interpret rgb
    let hex = "#00ff00";
    println!("The RGB value of {} is {:?}", hex, interpret_rgb(hex));

    // encode
    let s = "aaaabbbccd";
    println!("The encoded value of {} is {}", s, encode(s));

    // decode
    let s = "4a3b2c1d";
    println!("The decoded value of {} is {}", s, decode(s));
}

// calculate the median of a vector of integers
fn median(v: &mut Vec<i32>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort();
    let len = v.len();
    if len % 2 == 0 {
        let mid = len / 2;
        Some((v[mid] + v[mid - 1]) as f64 / 2.0)
    } else {
        Some(v[len / 2] as f64)
    }
}

// remove dupliicates from vector of integers
fn remove_duplicates(v: &mut Vec<i32>) {
    if v.is_empty() {
        return;
    }
    v.sort_unstable();
    v.dedup();
    // v.sort();

    // let mut i = 0;
    // while i < v.len() - 1 {
    //     if v[i] == v[i + 1] {
    //         v.remove(i);
    //     } else {
    //         i += 1;
    //     }
    // }
}

// remove duplicates from a vector of generic type
fn remove_duplicates_generic<T: Ord>(v: &mut Vec<T>) {
    if v.is_empty() {
        return;
    }
    v.sort_unstable();
    v.dedup();
}

// print any type of string
fn print_any_string<T: std::fmt::Display>(s: T) {
    println!("{}", s);
}

// case sensitive sort
fn case_sensitive_sort(v: &mut Vec<&str>) {
    v.sort();
}

// case insensitive sort
fn case_insensitive_sort(v: &mut Vec<&str>) {
    v.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
}
// deadlines - timestamp comparison
struct ImportantEvent {
    name: String,
    timestamp: DateTime<Local>,
}
trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.timestamp < Local::now()
    }
}

// sum a vector of optional integers
fn sum(v: &Vec<Option<i32>>) -> i32 {
    let mut sum = 0;
    for i in v {
        sum += i.unwrap_or(0);
    }
    sum

    // this also works
    // v.iter().map(|x| x.unwrap_or_default()).sum()
}

// calculate the number of weeks between two dates
// parameters are string slices of format "YYYY-MM-DD"
fn weeks_between(date1: &str, date2: &str) -> i64 {
    let date1 = NaiveDate::parse_from_str(date1, "%Y-%m-%d").unwrap();
    let date2 = NaiveDate::parse_from_str(date2, "%Y-%m-%d").unwrap();
    let duration = date2.signed_duration_since(date1);
    duration.num_weeks()
}

// given a path, check if file exists
fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

// interpret rgb hex color code
fn interpret_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}

// encode a string using run-length encoding
fn encode(s: &str) -> String {
    let mut encoded = String::new();
    let mut count = 1;
    let mut chars = s.chars();
    let mut last_char = chars.next().unwrap();
    for c in chars {
        if c == last_char {
            count += 1;
        } else {
            encoded.push_str(&format!("{}{}", count, last_char));
            count = 1;
            last_char = c;
        }
    }
    encoded.push_str(&format!("{}{}", count, last_char));
    encoded
}

// decode a string using run-length encoding
fn decode(s: &str) -> String {
    let mut decoded = String::new();
    let mut count_str = String::new();
    for c in s.chars() {
        if c.is_digit(10) {
            count_str.push(c);
        } else {
            let count = count_str.parse::<usize>().unwrap();
            decoded.push_str(&c.to_string().repeat(count));
            count_str.clear();
        }
    }
    decoded
}
