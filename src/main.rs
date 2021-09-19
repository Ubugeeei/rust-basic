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

    let _tuple = (1, "a");
    let _array = [1, 2, 3, 4];
    let _vec = vec![1, 2, 3, 4];

    dbg!(_string, _slice, _char, _isize, _usize, _float, _bite, _bin, _hex, _tuple, _array, _vec);

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

    // pattern math
    let flag = true;
    let message = match flag {
        true => "OK",
        false => "NG",
    };
    dbg!(message);

    // struct
    struct Animal {
        name: String,
    }
    let cat = Animal {
        name: String::from("cat"),
    };
    println!("cat name:{:?}", &cat.name);

    // method
    impl Animal {
        fn get_name_len(&self) -> usize {
            self.name.len()
        }
    }
    println!("cat name:{:?}", &cat.get_name_len());

    // enum
    #[derive(Debug)]
    enum ENUM {
        El1,
        El2,
    }
    let x = ENUM::El1;
    let y = ENUM::El2;
    dbg!(x, y);

    // Option
    let mut optional_num: Option<i32> = None;
    dbg!(optional_num);
    optional_num = Some(2);
    dbg!(optional_num.unwrap());
    // option & match
    let optional: Option<i32> = None;
    fn print_optional_value(optional: Option<i32>) {
        match optional {
            None => println!("no value!"),
            Some(i) => println!("value is {}", i),
        }
    }
    print_optional_value(optional);

    // generics
    fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    let res = add(1 as i128, 2 as i128);
    dbg!(res);
}
