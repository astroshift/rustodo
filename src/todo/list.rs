use std::fs;

pub fn list_entrys(_path: String) {
    let entrys = fs::read_to_string(_path)
        .expect("Something went wrong reading the file");

    print!("todo:\n{}", entrys);
}
