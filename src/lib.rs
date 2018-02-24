use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

const READ_ERR: &str = "something went wrong reading the file";
const WRITE_ERR: &str = "something went wrong writing the file";
const NOT_FOUND: &str = "file not found";

pub struct Fut {
    pub dup_map: HashMap<String, bool>,
}

impl Fut {
    pub fn new() -> Fut {
        Fut {
            dup_map: create_duplicate_map(),
        }
    }

    pub fn read_file_into_vec(self, filename: &str) -> Vec<String> {
        let mut target = vec![];

        for line in self.open_file_as_string(filename).split("\n") {
            target.push(String::from(line));
        }

        target
    }

    pub fn open_file_as_string(self, filename: &str) -> String {
        let mut file = File::open(filename).expect(NOT_FOUND);
        let mut contents = String::new();
        
        file.read_to_string(&mut contents).expect(READ_ERR);
        
        contents
    }

    pub fn write_file(self, source: String, target: &str) {
        let mut file = File::create(target).expect(NOT_FOUND);
        file.write_all(source.as_bytes()).expect(WRITE_ERR);
    }

    pub fn is_dup(&mut self, id: &str) -> bool {
        match self.dup_map.get(id) {
            Some(_) => true,
            None => {
                self.dup_map.insert(String::from(id), true);
                false
            }
        }
    }

    pub fn sub(self, source: &String, pattern: &str, replacement: &str) -> String {
        String::from(str::replace(&source, pattern, replacement))
    }
}

fn create_duplicate_map() -> HashMap<String, bool> {
    let dup_map: HashMap<String, bool> = HashMap::new();
    dup_map
}

#[cfg(test)]
mod tests {
    use Fut;

    #[test]
    fn it_stacks_lines_into_vec() {
        let fut = Fut::new();

        let string_vec: Vec<String> = fut.read_file_into_vec("./fixtures/test/test.csv");

        assert_eq!("hello,", string_vec[0]);
        assert_eq!("world,", string_vec[1]);
        assert_eq!("foo,", string_vec[2]);
        assert_eq!("bar", string_vec[3]);
    }

    #[test]
    fn it_opens_file_as_a_string() {
        let fut = Fut::new();

        let string_file: String = fut.open_file_as_string("./fixtures/test/test.csv");

        assert_eq!("hello,\nworld,\nfoo,\nbar\n", string_file);
    }

    #[test]
    fn it_replaces_string_contents() {
        let fut = Fut::new();

        let string: String = "hello\nworld\nfoo\nbar".to_string();

        assert_eq!(String::from("helloworldfoobar"), fut.sub(&string, "\n", ""));
    }

    #[test]
    fn it_can_create_a_duplicate_entry_if_id_not_found_and_find_duplicates() {
        let mut fut = Fut::new();

        // not duplicate
        assert!(!fut.is_dup("90"));

        // duplicate
        assert!(fut.is_dup("90"))
    }
}
