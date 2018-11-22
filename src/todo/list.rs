use std::fs;

/// Print the contents of a file
///
/// # Examples
///
/// let path = "/home/user/writefile";
///
/// list_entrys(path);

pub fn list_entrys(_path: &str) {
    let entrys = fs::read_to_string(_path)
        .expect("Something went wrong reading the file");

    print!("todo:\n{}", entrys);
}
