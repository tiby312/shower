//shower::make_answer!();

fn main() {
    let flower=&mut 5;
    let (val, k) = shower::python!(|| {
        for _ in 0..10 {
            println!("yoyoyoy {}",flower);
        }
        //Hello
        let k = 5;
        println!(" hello world example!!!{}", k);
        4
    });

    println!("source code=\n    {}", k);

    let val = val();

    println!("ret={}", val);

    // //let k: &str = k;

    // println!("<{}>", k);

    //println!("{}",h);
    //println!("{}",foo);
    //doop();
    //println!("{}",answer());
}
