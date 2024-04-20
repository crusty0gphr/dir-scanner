use std::process;

pub fn get_root() -> String {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("err: invalid arguments");
        process::exit(1);
    }
    args[1].to_owned()
}
