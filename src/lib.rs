use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_ascii(rgba: &[u8], width: usize, height: usize, out_w: usize, out_h: usize) -> String {
    let mut output = String::new();
    let chars = b".:-=+*#%@";
    let window_size_x = width / out_w;
    let window_size_y = height / out_h;

    for row in 0..out_h {
        for col in 0..out_w {
            let start_x = col * window_size_x;
            let start_y = row * window_size_y;
            let mut sum: f32 = 0.0;
            for x in 0..window_size_x {
                for y in 0..window_size_y {
                    let indx = ((start_y + y) * width + (start_x + x)) * 4;
                    let r = rgba[indx] as f32;
                    let g = rgba[indx + 1] as f32;
                    let b = rgba[indx + 2] as f32;
                    sum += r * 0.299 + g * 0.587 + b * 0.114;
                }
            }
            let avg = (sum / (window_size_x * window_size_y) as f32).clamp(0.0, 255.0);
            let char_indx = ((avg / 255.0) * (chars.len() - 1) as f32) as usize;
            let char_indx = char_indx.min(chars.len() - 1);
            output.push(chars[char_indx] as char);
        }
        output.push('\n');
    }

    output
}
