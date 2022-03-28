use num::complex::Complex;

//单元类型
fn main() -> () {
    greet_world();
    text_parse();
    basic_data_types();
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

    let z = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", z);

    fn forever() -> ! {
        loop {}
    }
    forever();
}
