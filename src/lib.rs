use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const READ_ERR: &str = "something went wrong reading the file";
const WRITE_ERR: &str = "something went wrong writing the file";
const NOT_FOUND: &str = "file not found";

/// Reads a file, and returns a Vec-String- where each element is a line.
pub fn read_file_into_line_vec(filename: &str) -> Vec<String> {
    let mut target: Vec<String> = vec![];

    for line in open_file_as_string(filename).split("\n") {
        target.push(String::from(line));
    }

    target
}

/// Reads a file, and returns a Vec-String- where each element is a character.
pub fn read_file_into_char_vec(filename: &str) -> Vec<String> {
    let mut target: Vec<String> = vec![];

    for line in open_file_as_string(filename).split("") {
        target.push(String::from(line));
    }

    target
}

/// Reads a file, and returns a String of the entire file.
pub fn open_file_as_string(filename: &str) -> String {
    let mut file = File::open(filename).expect(NOT_FOUND);
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect(READ_ERR);

    contents
}

/// Writes a new file from a String.
/// Target is just an &str for where the file will be written.
pub fn write_file(source: String, target: &str) {
    let mut file = File::create(target).expect(NOT_FOUND);
    file.write_all(source.as_bytes()).expect(WRITE_ERR);
}

/// Checks the duplicated map passed in for duplicates.
/// ```
/// extern crate fut;
///
/// use fut::{is_dup, create_duplicate_map};
///
/// let mut dup_map = create_duplicate_map();
///
/// if is_dup(&mut dup_map, "some_key") {
///   println!("We must do something about this duplicate!")
/// } else {
///   println!("Good to go!")
/// }
/// ```
///
pub fn is_dup(dup_map: &mut HashMap<String, bool>, id: &str) -> bool {
    match dup_map.get(id) {
        Some(_) => true,
        None => {
            dup_map.insert(String::from(id), true);
            false
        }
    }
}

/// Stores a new string in a HashMap where the values is a Vec-String- of all values.
pub fn store_into_map(
    store_map: &mut HashMap<String, Vec<String>>,
    key: &str,
    value: String,
) -> bool {
    let mut vals: Vec<String> = vec![];

    let exists = match store_map.get(key) {
        Some(values) => {
            vals = values.to_vec();
            true
        }
        None => false,
    };

    let mut new_values: Vec<String> = vec![];

    if exists == false {
        vals.push(value);
    }

    for val in vals {
        new_values.push(val.to_owned());
    }

    store_map.insert(String::from(key), new_values);

    exists
}

/// Simple find and replace util.
pub fn sub(source: &String, pattern: &str, replacement: &str) -> String {
    String::from(str::replace(&source, pattern, replacement))
}

/// A simple HashMap that store true or false to find duplicates.
pub fn create_duplicate_map() -> HashMap<String, bool> {
    let dup_map: HashMap<String, bool> = HashMap::new();
    dup_map
}

/// The expected HashMap that fut exposes.
/// Reading from CSVs or textfiles can often have duplicates.
/// So here we ensure that we can store all duplicates.
pub fn create_store_map() -> HashMap<String, Vec<String>> {
    let store_map: HashMap<String, Vec<String>> = HashMap::new();
    store_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_stacks_lines_into_vec() {
        let string_vec: Vec<String> = read_file_into_line_vec("./fixtures/test/test.csv");

        assert_eq!("hello,", string_vec[0]);
        assert_eq!("world,", string_vec[1]);
        assert_eq!("foo,", string_vec[2]);
        assert_eq!("bar", string_vec[3]);
    }

    #[test]
    fn it_stacks_chars_into_vec() {
        let string_vec: Vec<String> = read_file_into_char_vec("./fixtures/test/test.csv");

        assert_eq!("", string_vec[0]);
        assert_eq!("h", string_vec[1]);
        assert_eq!("e", string_vec[2]);
    }

    #[test]
    fn it_opens_file_as_a_string() {
        let string_file: String = open_file_as_string("./fixtures/test/test.csv");

        assert_eq!("hello,\nworld,\nfoo,\nbar\n", string_file);
    }

    #[test]
    fn it_replaces_string_contents() {
        let string: String = "hello\nworld\nfoo\nbar".to_string();

        assert_eq!(String::from("helloworldfoobar"), sub(&string, "\n", ""));
    }

    #[test]
    fn it_can_create_a_duplicate_entry_if_id_not_found_and_find_duplicates() {
        let mut dups = create_duplicate_map();

        // not duplicate
        assert!(!is_dup(&mut dups, "90"));

        // duplicate
        assert!(is_dup(&mut dups, "90"))
    }

    #[test]
    fn it_can_store_into_map() {
        let mut store = create_store_map();

        assert!(!store_into_map(
            &mut store,
            "1",
            "1,er,gh,45,epp".to_string()
        ));
        assert!(store_into_map(
            &mut store,
            "1",
            "1,as,vb,45,abb".to_string()
        ));
    }

    #[test]
    fn it_reads_and_writes_a_large_file() {
        let file = open_file_as_string("fixtures/test/test.csv");

        write_file(file, "fixtures/results/test.csv");

        assert!(true);
    }

}
