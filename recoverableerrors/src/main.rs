use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
fn main() {
    read_username_from_file_simplifiedpropagation();
    let f = File::open("hello.txt");
    ///Example of handling an error
    let f = match f{
        Ok(file) => file,
        Err(error) => {
            //If file not found, create new file
            match error.kind(){
                ErrorKind::NotFound => match File::create("hello.txt"){
                    //throw error if can't create
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    //Shortcut to handle errors with a panic
    let f = File::open("blah.txt").unwrap();
    //Shortcut to custom message with error info
    //let f = File::open("blah.txt").expect("Failed to open file");
}

/*
 * Another way to write it
 *  if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
  */
//propagating the error and letting it bubble up
  fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    //Will return early in event of an error
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

fn read_username_from_file_simplifiedpropagation() -> Result<String, io::Error>{
        //Question mark allows error to bubble up
        let mut f = File::open("blah.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)

        // fs::read_to_string("hello.txt") can technically just read file this way
    }
