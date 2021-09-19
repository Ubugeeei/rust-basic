fn main() {
    // data
    let _string = String::from("hello");
    let _slice = "hello";
    let _char = 'c';

    let _i32 = 5;
    let _i64: i64 = 3;
    let _isize: isize = -3;
    let _usize: usize = 3;

    let _float = 2.1;
    let _bite = b'a';
    let _bin = 0b1111_0000_1011_1110;
    let _hex = 0x34ae;
    dbg!(_string, _slice, _char, _isize, _usize, _float, _bite, _bin, _hex);

    // function
    fn func() {
        println!("hello from func");
    }
    func();

    fn func_args(massage: &str) {
        println!("message: {}", massage);
    }
    func_args("helllllo");

    fn func_return(a: isize, b: isize) -> isize {
        a + b
    }
    let result = func_return(1, 2);
    dbg!(result);

    // if statement
    let a = 2;
    if a % 2 == 0 {
        println!("even!!");
    } else {
        println!("odd!!");
    }

    // loop statement
    let mut count = 0;
    loop {
        count += 1;
        if count == 10 {
            break;
        }
    }
    dbg!(count);

    // while statement
    let mut count = 0;
    while count < 10 {
        count += 1;
    }
    dbg!(count);

    // for statement
    for el in [1, 2, 3, 4].iter() {
        dbg!(el);
    }

    for el in 1..5 {
        dbg!(el);
    }
}
