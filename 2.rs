fn main() {
    println!("{}", fib(0, 1));
}


fn fib(i: u32, j: u32) -> u32 {
    let k =
        if (i + j) % 2 == 0 {
            i + j
        } else {
            0
        };
    if i+j < 4u32*10u32.pow(6) {
        k + fib(j, i+j)
    } else {
        0u32
    }
}
