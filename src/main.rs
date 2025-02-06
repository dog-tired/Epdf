
use std::mem;
use std::io;
mod pdf_util;

fn main() {
    // 获取命令行参数的迭代器
    let args: Vec<String> = std::env::args().collect();

    // 打印参数数量
    // println!("参数数量: {}", args.len());

    // 遍历并打印每个参数
    let mut op = match args.get(1) {
        Some(arg) => arg,
        None => "",
    };

    // for (index, arg) in args.iter().enumerate() {
    //     println!("参数 {}: {}", index, arg);
    // }

    // 处理用户输入
    // .\Epdf.exe D:\ypj\rust\e-pdf\concat-test.pdf --dis
    match op {
        "--help" => {
           print_help();
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
        "--water" => {
            let path = match args.get(2) {
                Some(arg) => arg,
                None => "",
            };
            let w = match args.get(3) {
                Some(arg) => arg,
                None => "",
            };
            pdf_util::watermark(path, w);
        },
        "--2images" => {
            let path = match args.get(2) {
                Some(arg) => arg,
                None => "",
            };
            pdf_util::exportImages(path);
        },
        _ => println!("Invalid option, please try again."),
    }

}


fn print_help() {
    let exe_name = ".\\Epdf.exe";
    let dis_desc = format!("显示pdf ：{} --dis concat-test.pdf", exe_name);
    let copy_desc = format!("从原始pdf拷贝新的pdf ：{} --copy concat-test.pdf 1-2,2,2,2,2,1,1,1", exe_name);
    let water_desc = format!("嵌入水印 ：{} --water concat-test.pdf waterMark", exe_name);
    let images_desc = format!("另存为图片 ：{} --2images concat-test.pdf", exe_name);

    let commands = vec![
        ("--help", "显示帮助信息"),
        ("--dis", &dis_desc),
        ("--copy", &copy_desc),
        ("--water", &water_desc),
        ("--2images", &images_desc),
    ];

    let max_command_len = commands.iter().map(|(cmd, _)| cmd.len()).max().unwrap_or(0);

    println!("命令列表：");
    println!("{:-<80}", "");
    for (cmd, desc) in commands {
        println!("{:<width$} {}", cmd, desc, width = max_command_len + 2);
    }
    println!("{:-<80}", "");
}