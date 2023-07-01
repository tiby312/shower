//shower::make_answer!();

fn main() {
    let (k, src) = shower::source!({
        for i in 0..5 {
            println!("{}", i);
        }
        4
    });

    println!("source code:\n{}", src);

    println!("running program:");
    let k = k();

    println!("program returned={}", k);

    // println!("source code:\n    {}", k);

    // println!("Running program:");
    // let val = val();

    // println!("Program returned={}", val);
}
