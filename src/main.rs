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

    // RLE Encoding
    // b = white (>=128), o = black (<128), $ = newline, ! = end
    let raw_output = (0..height)
        .map(|y| {
            let runs: Vec<_> = (0..width)
                .map(|x| if gray_img.get_pixel(x, y)[0] >= 128 { 'b' } else { 'o' })
                .dedup_with_count()
                .collect();

            // If line is all white, return empty string
            if runs.len() == 1 && runs[0].1 == 'b' {
                String::new()
            } else {
                runs.into_iter()
                    .map(|(count, ch)| {
                        if count > 1 {
                            format!("{}{}", count, ch)
                        } else {
                            ch.to_string()
                        }
                    })
                    .collect::<String>()
            }
        })
        .join("$");

    // Compress consecutive '$'
    let mut output = String::with_capacity(raw_output.len());
    let mut dollar_count = 0;
    for ch in raw_output.chars() {
        if ch == '$' {
            dollar_count += 1;
        } else {
            if dollar_count > 0 {
                if dollar_count > 1 {
                    output.push_str(&dollar_count.to_string());
                }
                output.push('$');
                dollar_count = 0;
            }
            output.push(ch);
        }
    }
    if dollar_count > 0 {
        if dollar_count > 1 {
            output.push_str(&dollar_count.to_string());
        }
        output.push('$');
    }

    Ok(format!("{}!", output))
}
