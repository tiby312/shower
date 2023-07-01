
fn main() {
    let (k, src) = shower::source!(|| {
        for i in 0..5 {
            println!("{}", i);
        }
        "{abc}"
    });

    // print the source code of the program
    println!("{}", src);

    println!("running program:");
    let k = k();

    println!("program returned={}", k);

}
