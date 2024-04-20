mod scanner;
mod args;

use crate::scanner::{to_report, walk};
use crate::args::get_root;

fn main() {
    let root_dir = get_root();
    let (dir, err) = walk(root_dir);
    to_report(dir);
    let _ = err;
}
