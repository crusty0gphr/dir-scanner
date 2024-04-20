mod dir_scanner;

use dir_scanner::{errors, scanner};
use dir_scanner::args::get_root;

fn main() {
    let root_dir = get_root();
    let (dir, err) = scanner::walk(root_dir);
    scanner::to_report(dir);
    errors::to_report(err);
}
