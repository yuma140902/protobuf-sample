use crate::protos::person::Person;
use protobuf::Message;
use std::{fs::File, io::Read};

mod protos;

fn main() {
    // write to file
    let mut person1 = Person::new();
    person1.id = 42;
    person1.name = "Hoge hoge".to_owned();
    let mut file = File::create("person.bin").unwrap();
    person1.write_to_writer(&mut file).unwrap();

    // read from file
    let mut file = File::open("person.bin").unwrap();
    let mut buffer = vec![];
    file.read_to_end(&mut buffer).unwrap();
    let mut person2 = Person::new();
    person2.merge_from_bytes(&buffer).unwrap();

    assert_eq!(person1, person2);
    dbg!(person1);
    dbg!(person2);
}
