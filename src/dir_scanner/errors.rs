pub fn to_report(errors: Vec<walkdir::Error>) {
    if errors.len() == 0 {
        return;
    }
    println!("err");
    for error in errors {
        println!("\t - {}", error.to_string())
    }
}
