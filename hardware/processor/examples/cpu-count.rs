use num_cpus;

fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}
