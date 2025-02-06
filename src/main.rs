
use std::mem;
use std::io;
mod pdf_util;

fn main() {
    // 获取命令行参数的迭代器
    let args: Vec<String> = std::env::args().collect();

    // 打印参数数量
    println!("参数数量: {}", args.len());

    // 遍历并打印每个参数
    let mut op = match args.get(1) {
        Some(arg) => arg,
        None => "",
    };

    for (index, arg) in args.iter().enumerate() {
        println!("参数 {}: {}", index, arg);
    }

    // 处理用户输入
    // .\Epdf.exe D:\ypj\rust\e-pdf\concat-test.pdf --dis
    match op {
        "--help" => {
            println!("--help: 显示帮助信息");
            println!(r"--dis 显示pdf ：.\Epdf.exe --dis concat-test.pdf");
            println!(r"--copy 新建pdf ：.\Epdf.exe --copy D:\ypj\rust\e-pdf\concat-test.pdf 1-2,2,2,2,2,1,1,1");
        },
        "--dis" => {
            let path = match args.get(2) {
                Some(arg) => arg,
                None => "",
            };
            pdf_util::display_pdf(path);
        },
        "--copy" => {
            let path = match args.get(2) {
                Some(arg) => arg,
                None => "",
            };
            let pages = match args.get(3) {
                Some(arg) => arg,
                None => "",
            };
            pdf_util::copy(path, pages);
        },
        _ => println!("Invalid option, please try again."),
    }

}
