use druid::{
    widget::{Flex, Image as DruidImage, Label, Scroll, SizedBox}, Color, AppLauncher, ImageBuf, Widget, WidgetExt, WindowDesc
};
use pdfium_render::*;
use pdfium_render::prelude::*;
// 为 image 库重命名，避免冲突
use image as external_image;
use external_image::{DynamicImage, ImageFormat, ImageBuffer, Rgba};
use std::sync::Arc;
use std::io::Cursor;
use piet::*;
use std::fs;

/**
 * https://github.com/ajrcarey/pdfium-render/blob/master/examples/export.rs
 */

// 定义一个包装类型，方便存储图像数据
struct ImageWrapper {
    data: DynamicImage,
    format: ImageFormat,
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
pub fn exportImages(pdf_path: &str) -> Result<(), PdfiumError> {
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
        .set_maximum_height(2000)
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


/**
 * 提取文本元素

 */
pub fn extract_text(pdf_path: &str) -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    Pdfium::default()
        .load_pdf_from_file(pdf_path, None)?
        .pages()
        .iter()
        .enumerate()
        .for_each(|(index, page)| {
            // For each page in the document, output the text on the page to the console.

            println!("=============== Page {} ===============", index);

            println!("{}", page.text().unwrap().all());

            // PdfPageText::all() returns all text across all page objects of type
            // PdfPageObjectType::Text on the page - this is convenience function,
            // since it is often useful to extract all the page text in one operation.
            // We could achieve exactly the same result by iterating over all the page
            // text objects manually and concatenating the text strings extracted from
            // each object together, like so:

            // Extract all text on the page
            // let all_text = page.objects()
            //    .iter()
            //    .filter_map(|object| object
            //         .as_text_object()
            //         .map(|object| object.text()))
            //    .collect::<Vec<_>>()
            //    .join("");

            // // 假设每行最多显示 80 个字符
            // let line_length = 80;
            // for chunk in all_text.chars().collect::<Vec<char>>().chunks(line_length) {
            //     let line: String = chunk.iter().collect();
            //     println!("{}", line);
            // }
        });

    Ok(())
}



#[test]
fn test_exporter() {
    extract_images(PDF_PATH);
}

#[test]
fn test_ex_text() {
    extract_text(PDF_PATH);
}