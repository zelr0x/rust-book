mod stats;

fn main() {
    stats::print(&stats::get(&vec![1, 2, 8, 16, 32]).unwrap());
}
