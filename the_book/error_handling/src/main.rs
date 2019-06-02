use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let res1 = read_user_name_from_file1().unwrap();
    println!("{}", res1);

    let res2 = read_user_name_from_file2().unwrap();
    println!("{}", res2);

    let res3 = read_user_name_from_file3().unwrap();
    println!("{}", res3);

    let res4 = read_user_name_from_file4().unwrap();
    println!("{}", res4);
}

fn read_user_name_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("./src/hello.txt")
}

fn read_user_name_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    // It's OK to do method chaining!!
    File::open("./src/hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_user_name_from_file2() -> Result<String, io::Error> {
    // Question mark operater
    // https://doc.rust-lang.org/std/result/#the-question-mark-operator-
    // This simplifies "Return if ok, else Err" boiler plate pattern.
    // Can only be used in a fucntion which returns Result
    let mut f = File::open("./src/hello.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_user_name_from_file1() -> Result<String, io::Error> {
    let f = File::open("./src/hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
