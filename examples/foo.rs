//shower::make_answer!();

fn main() {
    let (k, src) = shower::source!(|| {
        for i in 0..5 {
            println!("{}", i);
        }
        4
    });

    k();

    println!("{}", src);

    // println!("source code:\n    {}", k);

    // println!("Running program:");
    // let val = val();

    // println!("Program returned={}", val);
}
