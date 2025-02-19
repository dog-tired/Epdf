
mod pdf_util;
mod license;
use core::num;

use pdf_util::PdfPageRange;

fn main() {
    // 获取命令行参数的迭代器
    let args: Vec<String> = std::env::args().collect();

    // 遍历并打印每个参数
    let mut op = match args.get(1) {
        Some(arg) => arg,
        None => "",
    };

    // 处理用户输入
    match op {
        "--help" => {
           print_help();
        },
        "--v" => {
           print_version();
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
        "--concat" => {
            let mut pdf_page_ranges = Vec::new();
            let mut i = 2;
            while i < args.len() {
                let file_path = args[i].clone();
                let page_range_str = args[i + 1].clone();
                pdf_page_ranges.push(PdfPageRange {
                    file_path,
                    page_indices: page_range_str,
                });
                i += 2;
            }
            pdf_util::concat(pdf_page_ranges);
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
            let width = match args.get(3) {
                Some(arg) => match arg.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => 1000,
                },
                None => 1000,
            };
            pdf_util::exportImages(path, width);
        },
        "--extract_images" => {
            let path = match args.get(2) {
                Some(arg) => arg,
                None => "",
            };

            pdf_util::extract_images(path);
        },
        "--extract_text" => {
            let path = match args.get(2) {
                Some(arg) => arg,
                None => "",
            };
            pdf_util::extract_text(path);
        },
        "--images_2_pdf" => {
            let path = match args.get(2) {
                Some(arg) => arg,
                None => "",
            };
            pdf_util::create_pdf_from_image(path);
        },
        _ => println!("Invalid option, please try again."),
    }

}


fn print_help() {
    let exe_name = "Epdf.exe";
    let dis_desc = format!("显示pdf ：{} --dis concat-test.pdf", exe_name);
    let copy_desc = format!("从原始pdf拷贝新的pdf ：{} --copy concat-test.pdf 1-2,2,2,2,2,1,1,1", exe_name);
    let concat_desc = format!("合并多个pdf文件 ：{} --concat pdf1_path page_selection1 pdf2_path page_selection2", exe_name);
    let water_desc = format!("嵌入水印 ：{} --water concat-test.pdf waterMark", exe_name);
    let images_desc = format!("另存为图片 ：{} --2images concat-test.pdf 1000", exe_name);
    let extract_images_desc: String = format!("提取图片元素 ：{} --extract_images concat-test.pdf", exe_name);
    let extract_txt_desc = format!("提取文字元素 ：{} --extract_text concat-test.pdf", exe_name);
    let images_2_pdf_desc = format!("文件夹中图片合成pdf（图片名称为数字，合成时按照序号顺序） ：{} --images_2_pdf floder_path", exe_name);



    let commands = vec![
        ("--help", "显示帮助信息"),
        ("--v", "显示版本"),
        ("--dis", &dis_desc),
        ("--copy", &copy_desc),
        ("--concat", &concat_desc),
        ("--water", &water_desc),
        ("--2images", &images_desc),
        ("--extract_images", &extract_images_desc),
        ("--extract_text", &extract_txt_desc),
        ("--images_2_pdf", &images_2_pdf_desc),
    ];

    let max_command_len = commands.iter().map(|(cmd, _)| cmd.len()).max().unwrap_or(0);

    println!("命令列表：");
    println!("{:-<80}", "");
    for (cmd, desc) in commands {
        println!("{:<width$} {}", cmd, desc, width = max_command_len + 2);
    }
    println!("{:-<80}", "");
}


fn print_version() {
    println!("authors: buRanXin");
    println!("Epdf version 1.0.0");
}