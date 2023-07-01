
Get the source code of a code block as a static string.

### Example

```rust
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
```

### Output

```
shower::source!(|| {
    for i in 0..5 {
        println!("{}", i);
    }
    "{abc}"
})
running program:
0
1
2
3
4
program returned={abc}
```