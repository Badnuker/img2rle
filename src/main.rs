use image::GenericImageView;
use itertools::Itertools;
use std::env;
use std::path::Path;
use std::process;

fn main() {
    match run() {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    
    // 获取参数
    let img_path = match args.as_slice() {
        [_, path, ..] => path,
        [program_name, ..] => return Err(format!("Usage: {} <image_path>", program_name)),
        [] => return Err("Usage: img2rle <image_path>".to_string()),
    };

    if !Path::new(img_path).exists() {
        return Err(format!("File '{}' not found.", img_path));
    }

    // 处理图片打开结果
    let img = match image::open(img_path) {
        Ok(img) => img,
        Err(e) => return Err(format!("Failed to open image: {}", e)),
    };

    let (width, height) = img.dimensions();
    eprintln!("Processing image: {} ({}x{})", img_path, width, height);

    if width == 0 || height == 0 {
        return Err("Empty image.".to_string());
    }

    let gray_img = img.to_luma8();

    // RLE 编码
    let output = (0..height)
        .map(|y| {
            (0..width)
                .map(|x| if gray_img.get_pixel(x, y)[0] >= 128 { 'b' } else { 'o' })
                .dedup_with_count()
                .map(|(count, ch)| {
                    if count > 1 {
                        format!("{}{}", count, ch)
                    } else {
                        ch.to_string()
                    }
                })
                .collect::<String>()
        })
        .join("$");

    Ok(format!("{}!", output))
}
