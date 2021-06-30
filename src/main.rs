fn main() {
    println!("Hello, world!");
}

fn print() -> &'static str {
    "Hello World!!!"
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::fs;
    use std::process::Command;

    #[test]
    fn test_main()
    {//&["/C", "echo hello"]
        fs::write("test_main.txt", print()).expect("Unable to write file");
        let output = Command::new("git")
            .args(["--no-pager",  "diff",  "test_main.txt"])
            .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)

        });
        assert_eq!("", String::from_utf8(output.stdout).unwrap());
    }
}

/*
fn main() {
    let data = "Some data!";
    fs::write("/tmp/foo", data).expect("Unable to write file");
}
 */