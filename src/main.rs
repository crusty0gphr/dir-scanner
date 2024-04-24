mod dir_scanner;

use dir_scanner::args::get_root;
use dir_scanner::{errors, scanner};

fn main() {
    let root_dir = get_root();
    let (dir, err) = scanner::walk(root_dir);
    scanner::to_report(dir);
    errors::to_report(err);
}
