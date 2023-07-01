//shower::make_answer!();

fn main() {
    shower::source!(k, src, || {
        for i in 0..5 {
            println!("{}", i);
        }
        4
    });

    //k();

    let a=unindent::unindent(&src);
    println!("{}", a);

    k();

    // println!("source code:\n    {}", k);

    // println!("Running program:");
    // let val = val();

    // println!("Program returned={}", val);
}
