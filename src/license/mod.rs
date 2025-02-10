
const LICENCE_FILE: &str = "licence.txt";


mod licence_mod {
    use std::fs::{self, File};
    use std::io::{self, Read, Write};
    use std::time::Instant;

    const LICENCE_PROMPT: &str = "The number of times has been used up. Please contact the author for activation. 1 yuan and 10000 times";


    pub fn get_content(file_path: &str) -> Result<String, io::Error> {
        // 打开指定路径的文件
        let mut file = File::open(file_path)?;
        // 定义一个可变的 String 类型变量，用于存储文件内容
        let mut content = String::new();
        // 读取文件内容到 content 中
        file.read_to_string(&mut content)?;
        println!("Reading file content: {} \n", content);
        Ok(content)
    }

    // 定义一个函数，用于读取文件内容并与 "123" 进行比较
    pub fn compare_file_content_with_123(file_path: &str) -> io::Result<bool> {
        print!("Reading file content...\n");
        // 打开指定路径的文件
        let mut file = File::open(file_path)?;
        // 定义一个可变的 String 类型变量，用于存储文件内容
        let mut content: String = String::new();
        println!("Reading file content: {} \n", content);
        // 读取文件的全部内容到 content 变量中
        file.read_to_string(&mut content)?;

        // 去除字符串首尾的空白字符
        let trimmed_content = content.trim();
        // 将处理后的内容与 "123" 进行比较，并返回比较结果
        Ok(trimmed_content == "123")
    }


    // 新增函数，每次调用将文件中的数字减 1
    pub fn decrement_file_number(file_path: &str) -> io::Result<()> {

        let start_time = Instant::now();
        // 读取文件内容
        let content = fs::read_to_string(file_path)?;
        let trimmed_content = content.trim();

        // 将内容解析为整数
        let mut number: i32 = trimmed_content.parse::<i32>().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "File content is not a valid integer",
            )
        })?;

        // 数字减 1
        number -= 1;

        // 判断数字是否小于 0
        if number < 0 {
            println!("{}", LICENCE_PROMPT);
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                LICENCE_PROMPT,
            ));
        }

        // 将更新后的数字写回文件
        fs::write(file_path, number.to_string())?;

        let elapsed_time = start_time.elapsed();
        println!("decrement_file_number 函数执行耗时: {:?}", elapsed_time);
        Ok(())
    }
}

mod tests {
    use super::licence_mod;

    
    const LICENCE_FILE: &str = "licence.txt";

    #[test]
    fn test_get_content() {
        match licence_mod::get_content("licence.txt") {
            Ok(content) => println!("文件内容: {}", content),
            Err(e) => eprintln!("读取文件时发生错误: {}", e),
        }
    }

    #[test]
    fn test_compare_file_content_with_123() {
        // 测试 compare_file_content_with_123 函数
        assert!(licence_mod::compare_file_content_with_123("license.txt").unwrap());
    }

    #[test]
    fn test_decrement_file_number() {
        // 测试 decrement_file_number 函数
        assert!(licence_mod::decrement_file_number("license.txt").is_ok());
    }
}