use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const READ_ERR: &str = "something went wrong reading the file";
const WRITE_ERR: &str = "something went wrong writing the file";
const NOT_FOUND: &str = "file not found";

// public

pub fn read_file_into_vec(filename: &str) -> Vec<String> {
    let mut target = vec![];

    for line in open_file_as_string(filename).split("\n") {
        target.push(String::from(line));
    }

    target
}

pub fn open_file_as_string(filename: &str) -> String {
    let mut file = File::open(filename).expect(NOT_FOUND);
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect(READ_ERR);

    contents
}

pub fn write_file(source: String, target: &str) {
    let mut file = File::create(target).expect(NOT_FOUND);
    file.write_all(source.as_bytes()).expect(WRITE_ERR);
}

pub fn is_dup(dup_map: &mut HashMap<String, bool>, id: &str) -> bool {
    match dup_map.get(id) {
        Some(_) => true,
        None => {
            dup_map.insert(String::from(id), true);
            false
        }
    }
}

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

    let mut new_values = vec![];

    if exists == false {
        vals.push(value);
    }

    for val in vals {
        new_values.push(val.to_owned());
    }

    store_map.insert(String::from(key), new_values);

    exists
}

pub fn sub(source: &String, pattern: &str, replacement: &str) -> String {
    String::from(str::replace(&source, pattern, replacement))
}

pub fn create_duplicate_map() -> HashMap<String, bool> {
    let dup_map: HashMap<String, bool> = HashMap::new();
    dup_map
}

pub fn create_store_map() -> HashMap<String, Vec<String>> {
    let store_map: HashMap<String, Vec<String>> = HashMap::new();
    store_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_stacks_lines_into_vec() {
        let string_vec: Vec<String> = read_file_into_vec("./fixtures/test/test.csv");

        assert_eq!("hello,", string_vec[0]);
        assert_eq!("world,", string_vec[1]);
        assert_eq!("foo,", string_vec[2]);
        assert_eq!("bar", string_vec[3]);
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
        let file = open_file_as_string("fixtures/something.csv");

        write_file(String::from(file), "fixtures/result/test.csv");

        assert!(true);
    }

}
