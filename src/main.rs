use num::complex::Complex;

//单元类型
fn main() -> () {
    greet_world();
    text_parse();
    basic_data_types();
    basic_data_types_test();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn greet_world() {
    let chinese = "你好，世界";
    let english = "hello world";
    let regions = [chinese, english];
    for region in regions.iter() {
        println!("{}---{}", region, region);
    }
}

fn text_parse() {
    let text = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";
    let records = text.lines();
    for (_, record) in records.enumerate() {
        if record.is_empty() {
            continue;
        }
        let fields: Vec<_> = record.split(",").map(|s| s.trim()).collect();
        if cfg!(debug_assertions) {
            eprintln!("{:?}", fields);
        }
        if fields.len() != 2 {
            continue;
        }
        let name = fields[0];
        let length = fields[1];
        if let Ok(value) = length.parse::<f32>() {
            println!("{}---{}", name, value);
        }
    }
}

fn basic_data_types() {
    let one_million: usize = 1_000_000;
    let pow: usize = one_million.pow(3);
    println!("one_millon.pow = {}", pow);
    let a = 'w'..='z';
    for i in a {
        println!("{}", i);
    }
    let b = Complex::new(1.0, 2.0);
    println!("{} + {}i", b.re, b.im);

    let x = "中国";
    let y = '中';
    println!(
        "占用了x {} y {}字节的内存大小",
        std::mem::size_of_val(&x),
        std::mem::size_of_val(&y)
    );
    //表达式
    let z = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", z);
    // 用不返回的函数
    // fn forever() -> ! {
    //     loop {}
    // }
    // forever();
    let s = "中国人";
    let v = vec![1, 2, 3];
    let _ = &v[1..2];
    let a = &s[0..3];
    println!("The value of a is: {}", a);
    let s1 = String::from("hello,world!");
    let _ = &s1[0..3];

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let email = "anotheremail@example.com";
    user1.email = (&email[1..2]).to_string();
    user1.username = (&email[1..2]).to_string();
    user1.active = true;
    user1.sign_in_count = 2;
}

fn basic_data_types_test() {
    // 编译器自动推导出one的类型
    let one = [1, 2, 3];
    // 显式类型标注
    let two = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    // 借用arrays的元素用作循环中
    for a in &arrays {
        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }

    println!("\t({:?})", arrays);
}
