use druid::{
    text, widget::{Flex, Image as DruidImage, Label, Scroll, SizedBox}, AppLauncher, Color, ImageBuf, Widget, WidgetExt, WindowDesc
};
use pdfium_render::prelude::*;
// 为 image 库重命名，避免冲突
use image as external_image;
use external_image::{DynamicImage, ImageFormat};
use pdfium_render::prelude::PdfPageObjectCommon;
use piet::*;
use std::fs;
use std::path::Path;

/**
 * https://github.com/ajrcarey/pdfium-render/blob/master/examples/export.rs
 */

// 定义一个包装类型，方便存储图像数据
struct ImageWrapper {
    data: DynamicImage,
    format: ImageFormat,
}

pub struct PdfPageRange {
    pub file_path: String,
    pub page_indices: String,
}

impl ImageWrapper {
    fn from_dynamic_image(image: DynamicImage, format: ImageFormat) -> Self {
        ImageWrapper { data: image, format }
    }
}

// test pdfPath 
const PDF_PATH: &str = "C:\\Users\\slh\\Downloads\\kafka.pdf";

// 从 PDF 加载页面图像
fn pdf_load_main(pdf_path: &str) -> Result<Vec<DynamicImage>, PdfiumError> {
    // let pdfium_result: Result<Box<dyn PdfiumLibraryBindings>, PdfiumError> = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"));
    // let pdfium = match pdfium_result {
    //     Ok(bindings) => Pdfium::new(bindings),
    //     Err(e) => return Err(e),
    // };
    let pdfium = Pdfium::default();

    // let pdf_path = "C:\\Users\\slh\\Downloads\\kafka.pdf";
    let document_result = pdfium.load_pdf_from_file(pdf_path, None);

    let render_config = PdfRenderConfig::new()
       .set_target_width(1000)
       .set_maximum_height(1000)
       .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    let mut page_images = Vec::new();

    match document_result {
        Ok(doc) => {
            for (_, page) in doc.pages().iter().enumerate() {
                let image = page.render_with_config(&render_config)?
                   .as_image()
                   .to_owned();
                page_images.push(image);
            }
        }
        Err(e) => {
            return Err(e);
        }
    }

    Ok(page_images)
}

// 将 DynamicImage 转换为 Druid 的 ImageBuf
fn dynamic_image_to_image_buf(image: &DynamicImage) -> ImageBuf {
    let rgba_image = image.to_rgba8();
    let width = rgba_image.width() as usize;
    let height = rgba_image.height() as usize;
    let data = rgba_image.into_raw();
    println!("width: {}, height: {}, data: {}", width, height, data.len());

    ImageBuf::from_raw(data, piet::ImageFormat::RgbaSeparate, width, height)
}


// #[test]
pub fn display_pdf(pdf_path: &str) {
    let page_images = match pdf_load_main(pdf_path) {
        Ok(images) => images,
        Err(e) => {
            eprintln!("Error loading PDF: {}", e);
            return;
        }
    };

    // 定义行间距和列间距
    let row_spacing = 10.0; // 垂直布局中每行之间的间距
    let col_spacing = 10.0; // 水平布局中每个组件之间的间距
    // 定义每行显示的图片数量
    let images_per_row = 3;
    // 定义固定容器的大小
    let container_width = 300.0;
    let container_height = 300.0;
    // 定义左右内边距
    let horizontal_padding = 1.0;

    // 创建一个垂直布局，用于存放每行的水平布局
    let mut main_column = Flex::column();

    let mut current_row = Flex::row();
    let mut image_count = 0;

    // 将每个图像添加到布局中
    for (index, image) in page_images.iter().enumerate() {
        let image_buf = dynamic_image_to_image_buf(image);
        let image_widget = DruidImage::new(image_buf);
        // 创建固定大小的容器并将图片组件放入其中
        let container = SizedBox::new(image_widget)
           .fix_width(container_width)
           .fix_height(container_height)
           .padding((horizontal_padding, 0.0))
           .border(Color::rgb8(192, 192, 192), 1.0);

        // 创建显示页码的 Text 组件
        let page_number = index + 1;
        let page_number_text = Label::new(format!("Page {}", page_number))
        .border(Color::rgb8(192, 192, 192), 1.0);

        // 创建一个水平布局，将图片容器和页码 Text 组件组合
        let image_with_page_number = Flex::column()
           .with_child(container)
           .with_spacer(col_spacing) // 设置图片容器和页码之间的间距
           .with_child(page_number_text)
           .padding((horizontal_padding, 0.0)) // 设置左右内边距
           .border(Color::rgb8(192, 192, 192), 1.0);

        current_row.add_spacer(row_spacing);
        current_row.add_child(image_with_page_number);
        image_count += 1;

        if image_count % images_per_row == 0 {
            main_column.add_child(current_row);
            main_column.add_spacer(row_spacing); // 设置行与行之间的间距
            current_row = Flex::row();
            image_count = 0;
        }
    }

    // 如果最后一行还有剩余图片，添加到主布局中
    main_column.add_child(current_row);

    // 添加滚动功能
    let scrollable = Scroll::new(main_column);

    // 创建窗口描述
    let main_window = WindowDesc::new(scrollable)
       .title("PDF Preview")
       .window_size((1000.0, 800.0));

    // 启动应用程序
    AppLauncher::with_window(main_window)
       .launch(())
       .expect("Failed to launch application");
}

// #[test]
pub fn exportImages(pdf_path: &str, maximum_width: i32) -> Result<(), PdfiumError> {
    // let bindings = Pdfium::bind_to_library(
    //     // Attempt to bind to a pdfium library in the current working directory...
    //     Pdfium::pdfium_platform_library_name_at_path("./"),
    // )
    // .or_else(
    //     // ... and fall back to binding to a system-provided pdfium library.
    //     |_| Pdfium::bind_to_system_library(),
    // )?;

    // let pdfium = Pdfium::new(bindings);
    let pdfium = Pdfium::default();

    // 定义要保存图片的文件夹路径
    let folder_path = "export-images";

    // 创建保存图片的文件夹，如果文件夹已存在则忽略错误
    fs::create_dir_all(folder_path).expect("Failed to create directory");

    // This pattern is common enough that it is the default constructor for the Pdfium struct,
    // so we could have also simply written:

    // let pdfium = Pdfium::default();

    // Next, we create a set of shared settings that we'll apply to each page in the
    // sample file when rendering. Sharing the same rendering configuration is a good way
    // to ensure homogenous output across all pages in the document.

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_width(maximum_width)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    // Load the sample file...

    let document = pdfium.load_pdf_from_file(pdf_path, None)?;

    // ... and export each page to a JPEG in the current working directory,
    // using the rendering configuration we created above.

    for (index, page) in document
        .pages()
        .iter() // ... get an iterator across all pages ...
        .enumerate()
    {
        let result = page
            .render_with_config(&render_config)? // Initializes a bitmap with the given configuration for this page ...
            .as_image() // ... renders it to an Image::DynamicImage ...
            .into_rgb8() // ... sets the correct color space ...
            .save_with_format(format!("export-images/export-page-{}.jpg", index), ImageFormat::Jpeg); // ... and exports it to a JPEG.
        println!("Successfully export the {} page", index);
        assert!(result.is_ok());
    }

    Ok(())
}


/**
 * 测试水印功能
 * 
 */
// #[test]
pub fn watermark(pdf_path: &str, w: &str) -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    let mut document = pdfium.load_pdf_from_file(pdf_path, None)?;

    // Add a page number and a large text watermark to every page in the document.

    let font = document.fonts_mut().helvetica();

    document.pages().watermark(|group, index, width, height| {
        // Create a page number at the very top of the page.

        let mut page_number = PdfPageTextObject::new(
            &document,
            format!("Page {}", index + 1),
            font,
            PdfPoints::new(14.0),
        )?;

        page_number.set_fill_color(PdfColor::GREEN)?;

        page_number.translate(
            (width - page_number.width()?) / 2.0, // Horizontally center the page number...
            height - page_number.height()?,       // ... and vertically position it at the page top.
        )?;

        group.push(&mut page_number.into())?;

        // Create a large text watermark in the center of the page.

        let mut watermark =
            PdfPageTextObject::new(&document, w, font, PdfPoints::new(100.0))?;

        watermark.set_fill_color(PdfColor::DARK_SLATE_GRAY.with_alpha(50))?;
        watermark.rotate_counter_clockwise_degrees(45.0)?;
        watermark.translate(
            (width - watermark.width()?) / 2.0 + PdfPoints::new(75.0),
            (height - watermark.height()?) / 2.0,
        )?;

        group.push(&mut watermark.into())?;

        Ok(())
    })?;

    document.save_to_file("watermark.pdf")
}


// https://github.com/ajrcarey/pdfium-render/blob/master/examples/concat.rs
// "1-2,2,2,2,2,1,1,1"
// #[test]
pub fn copy(pdf_path: &str, pages: &str) -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    // There are several functions available to copy one or more pages from one document
    // to another:

    // PdfDocument::append(): this is the simplest. It copies all pages in one document
    // into this PdfDocument, placing the copied pages at the end of this PdfDocument's
    // PdfPages collection.

    // PdfPages::import_page_from_document(): copies one page from a document
    // into this PdfPages collection at a user-defined position.

    // PdfPages::import_page_range_from_document(): copies multiple pages, expressed
    // as a sequential 0-indexed inclusive range, from a document into this PdfPages
    // collection at a user-defined position.

    // PdfPages::import_pages_from_document(): copies multiple pages, expressed as
    // a "human-friendly" 1-indexed comma-delimited string of page numbers and ranges,
    // from a document into this PdfPages collection at a user-defined position.
    // The page range string is the same as what you'd expect to use in, e.g. a
    // Print File dialog box, with a specification like "1,3-4,6,9-12" being accepted.

    // All these functions are demonstrated below.

    // Create a new blank document...

    let mut document = pdfium.create_new_pdf()?;

    // ... append all pages from a test file using PdfDocument::append() ...

    // document
    //     .pages_mut()
    //     .append(&pdfium.load_pdf_from_file("test/text-test.pdf", None)?)?;

    // ... import some more pages from another test file, this time
    // using PdfPages::import_pages_from_document() ...

    let destination_page_index = document.pages().len();

    document.pages_mut().copy_pages_from_document(
        &pdfium.load_pdf_from_file(pdf_path, None)?,
        pages, // Note: 1-indexed, not 0-indexed
        destination_page_index,
    )?;

    // // ... import some more pages from yet another test file, this time
    // // using PdfPages::import_page_range_from_document() ...

    // let destination_page_index = document.pages().len();

    // document.pages_mut().copy_page_range_from_document(
    //     &pdfium.load_pdf_from_file("test/form-test.pdf", None)?,
    //     0..=2, // Note: 0-indexed, inclusive range
    //     destination_page_index,
    // )?;

    // // ... insert front and back cover pages, this time using PdfPages::import_page_from_document() ...

    // document.pages_mut().copy_page_from_document(
    //     &pdfium.load_pdf_from_file("test/export-test.pdf", None)?,
    //     0, // First page, i.e. front cover; note: 0-indexed
    //     0,
    // )?;

    // let destination_page_index = document.pages().len();

    // document.pages_mut().copy_page_from_document(
    //     &pdfium.load_pdf_from_file("test/export-test.pdf", None)?,
    //     6, // Last page, i.e. back cover; note: 0-indexed
    //     destination_page_index,
    // )?;

    // ... remove the sixth page ...

    // document
    //     .pages()
    //     .get(5)? // 0-indexed
    //     .delete()?;

    // ... and save the final result.

    document.save_to_file("concat-test.pdf")
}


/**
 * 提取图片元素
 */
pub fn extract_images(pdf_path: &str) -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    Pdfium::default()
        .load_pdf_from_file(pdf_path, None)?
        .pages()
        .iter()
        .enumerate()
        .for_each(|(page_index, page)| {
            // For each page in the document, output the images on the page to separate files.

            println!("=============== Page {} ===============", page_index);

            page.objects()
                .iter()
                .enumerate()
                .for_each(|(object_index, object)| {
                    if let Some(image) = object.as_image_object() {
                        if let Ok(image) = image.get_raw_image() {
                            println!("Exporting image with object index {} to file", object_index);

                            let save_result = image.save_with_format(
                                format!(
                                    "page-{}-image-{}.png",
                                    page_index, object_index
                                ),
                                ImageFormat::Png,
                            );

                            if let Err(err) = save_result {
                                eprintln!("Failed to save image: {:?}", err);
                            }
                        }
                    }
                });
        });

    Ok(())
}



/// 提取文本元素
pub fn extract_text(pdf_path: &str) -> Result<(), PdfiumError> {
       // Load the PDF document
    let pdfium = Pdfium::default();
    let document = pdfium.load_pdf_from_file(pdf_path, None)?;

    // Iterate through each page in the document
    document.pages().iter().enumerate().for_each(|(index, page)| {
        // Output the text on the page to the console
        println!("=============== Page {} ===============", index);

        // Get all text characters on the page
        let page_text = page.text().unwrap();
        let chars = page_text.chars();

        // Group characters by their y-coordinate (line)
        let mut lines: Vec<Vec<(char, PdfPoints, PdfPoints)>> = Vec::new();

        for char in chars.iter() {
            let mut unicode_char = char.unicode_char().unwrap();
            let x = char.origin_x().unwrap();
            let y = char.origin_y().unwrap();
            // println!("{}, x: {}, y: {}", unicode_char, x.value, y.value);
            if (unicode_char == '\n' || unicode_char == '\r') {
                // println!("this is n: {}, {}", x, y);
                unicode_char = ' ';
            }
            // Find the line that this character belongs to
            let mut found_line = false;
            for line in &mut lines {
                // If the y-coordinate is close enough to an existing line, add the character to that line
                if (line[0].2.value - y.value).abs() < 1.0 {
                    line.push((unicode_char, x, y));
                    found_line = true;
                    break;
                }
            }

            // If no suitable line is found, create a new line
            if !found_line {
                lines.push(vec![(unicode_char, x, y)]);
            }
        }

        // Sort lines by their y-coordinate (top to bottom)
        // lines.sort_by(|a, b| a[0].2.value.partial_cmp(&b[0].2.value).unwrap());
        lines.sort_by(|a, b| b[0].2.value.partial_cmp(&a[0].2.value).unwrap());


        // Output each line
        for line in lines {
            // Sort characters in each line by their x-coordinate (left to right)
            let mut sorted_line = line;
            sorted_line.sort_by(|a, b| a.1.value.partial_cmp(&b.1.value).unwrap());

            // Collect characters into a string and print the line
            let line_text: String = sorted_line.iter().map(|c| c.0).collect();
            println!("{}", line_text);
        }
    });

    Ok(())
}


pub fn concat(pdf_page_ranges: Vec<PdfPageRange>) -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    // There are several functions available to copy one or more pages from one document
    // to another:

    // PdfDocument::append(): this is the simplest. It copies all pages in one document
    // into this PdfDocument, placing the copied pages at the end of this PdfDocument's
    // PdfPages collection.

    // PdfPages::import_page_from_document(): copies one page from a document
    // into this PdfPages collection at a user-defined position.

    // PdfPages::import_page_range_from_document(): copies multiple pages, expressed
    // as a sequential 0-indexed inclusive range, from a document into this PdfPages
    // collection at a user-defined position.

    // PdfPages::import_pages_from_document(): copies multiple pages, expressed as
    // a "human-friendly" 1-indexed comma-delimited string of page numbers and ranges,
    // from a document into this PdfPages collection at a user-defined position.
    // The page range string is the same as what you'd expect to use in, e.g. a
    // Print File dialog box, with a specification like "1,3-4,6,9-12" being accepted.

    // All these functions are demonstrated below.

    // Create a new blank document...

    let mut document = pdfium.create_new_pdf()?;

    // ... append all pages from a test file using PdfDocument::append() ...

    for pdf_page_range in pdf_page_ranges {
        let source_doc = pdfium.load_pdf_from_file(&pdf_page_range.file_path, None)?;
        let destination_page_index = document.pages().len();
        document.pages_mut().copy_pages_from_document(
            &source_doc,
            &pdf_page_range.page_indices,
            destination_page_index,
        )?;
    }


    // document
    //     .pages_mut()
    //     .append(&pdfium.load_pdf_from_file("test/text-test.pdf", None)?)?;

    // // ... import some more pages from another test file, this time
    // // using PdfPages::import_pages_from_document() ...

    // let destination_page_index = document.pages().len();

    // document.pages_mut().copy_pages_from_document(
    //     &pdfium.load_pdf_from_file("test/export-test.pdf", None)?,
    //     "3-6", // Note: 1-indexed, not 0-indexed
    //     destination_page_index,
    // )?;

    // ... and save the final result.

    document.save_to_file("concat.pdf")
}



use image::GenericImageView;
use regex::Regex;

fn images_to_pdf(images: Vec<String>, output_path: &str) -> Result<(), PdfiumError> {
    // 初始化 Pdfium 库
    let pdfium = Pdfium::default();
    // 创建一个新的 PDF 文档
    let mut document = pdfium.create_new_pdf()?;

    for (index, image_path) in images.iter().enumerate() {
        // 打开图片
        let img: DynamicImage = match image::open(image_path) {
            Ok(img) => {
                println!("success");
                img
            },
            Err(e) => {
                eprintln!("Failed to open image {}: {}, Please check whether the file format is the same as the suffix name", image_path, e);
                continue;
            }
        };

        let (width, height) = img.dimensions();
        println!("width: {}, height: {}", width, height);

        // 创建一个新的 PDF 页面
        let mut page = document
            .pages_mut()
            .create_page_at_end(PdfPagePaperSize::a4())?;

        // 将图片渲染为位图
        let bitmap = img.to_rgba8();

        // 计算图片在 A4 页面上的缩放比例，以确保图片完整显示
        let page_width = page.width().value;
        let page_height = page.height().value;
        let scale_factor = (page_width / width as f32).min(page_height / height as f32);
        let scaled_width = width as f32 * scale_factor;
        let scaled_height = height as f32 * scale_factor;

        // 创建一个新的 PdfPageImageObject 实例，使用转换后的位图
        let mut image_object = PdfPageImageObject::new_with_width(
            &document,
            &img,
            PdfPoints::new(scaled_width),
        )?;

        // 设置图片在页面上的位置，使其居中显示
        let x = (page_width - scaled_width) / 2.0;
        let y = (page_height - scaled_height) / 2.0;
        image_object.translate(PdfPoints::new(x), PdfPoints::new(y))?;

        // 将图片对象添加到页面
        page.objects_mut().add_image_object(image_object)?;
    }

    // 保存生成的 PDF 文件
    document.save_to_file(output_path)?;

    Ok(())
}


fn read_folder(folder_path: &str) -> Result<Vec<String>, std::io::Error> {
    // 读取文件夹中的所有条目
    let entries = fs::read_dir(folder_path)?;
    let mut file_paths: Vec<String> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                Some(path.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    // 按文件名中的数字排序
    let re = Regex::new(r"\d+").unwrap(); // 匹配文件名中的数字
    file_paths.sort_by(|a, b| {
        let a_num = extract_number(&re, a);
        let b_num = extract_number(&re, b);
        a_num.cmp(&b_num)
    });

    Ok(file_paths)
}

// 提取文件名中的数字
fn extract_number(re: &Regex, file_path: &str) -> i32 {
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");

    // 查找文件名中的数字部分
    if let Some(captures) = re.find(file_name) {
        captures.as_str().parse::<i32>().unwrap_or(0)
    } else {
        0
    }
}


pub fn create_pdf_from_image(folder_path: &str) {
    // 输入文件夹路径
    // let folder_path = "D:/ypj/vs_note/downloaded_images"; // 请替换为实际的文件夹路径

    // 读取文件夹中的文件
    let read_folder_result = read_folder(folder_path);
    match read_folder_result {
        Ok(images) => {
            // 打印组装后的数组
            println!("let images = {:?};", images);
            let output_path = "output.pdf";

            if let Err(e) = images_to_pdf(images, output_path) {
                eprintln!("Failed to generate PDF: {}", e);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}


#[test]
fn test_exportImages() {
    exportImages(PDF_PATH, 1000);
}

#[test]
fn test_exporter() {
    extract_images(PDF_PATH);
}

#[test]
fn test_ex_text() {
    extract_text(PDF_PATH);
}

#[test]
fn main2() {
    let path = "D:/ypj/vs_note/downloaded_images";
    create_pdf_from_image(path);
}