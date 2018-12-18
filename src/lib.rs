#[cfg(test)]
use std::fs::File;
use std::io::Read;

#[test]
fn get_application_list() {
  let mut application_list = String::new();
  let mut obtained_from_list = String::new();
  let mut file = File::open("test_application_list").unwrap();

  file.read_to_string(&mut application_list).unwrap();

  let mut file = File::open("test_obtained_from_list").unwrap();

  file.read_to_string(&mut obtained_from_list).unwrap();
}
