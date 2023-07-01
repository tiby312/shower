//shower::make_answer!();

fn main() {
    let (val, k) = shower::source!(|| {
        for i in 0..5 {
            println!("{}", i);
        }
        4
    });

    println!("source code:\n    {}", k);

    println!("Running program:");
    let val = val();

    println!("Program returned={}", val);
}
