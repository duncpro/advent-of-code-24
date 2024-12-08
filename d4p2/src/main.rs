fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let char_at = |x, y| input.as_bytes()[y * (width + 1) + x] as char;

    let mut scratch = String::new();
    let mut total: usize = 0;
    
    for center_x in 0..width {
        for center_y in 0..height {
            let mut nslope_matched = false;
            let mut pslope_matched = false;
            
            scratch.clear();
            'nslope: {
                for i in -1isize..=1 {
                    if let Some(x) = center_x.checked_add_signed(i) {
                        if let Some(y) = center_y.checked_add_signed(i) {
                            if x >= width || y >= height { break 'nslope; }
                            scratch.push(char_at(x, y));
                        }
                    }
                }
                nslope_matched = ["MAS", "SAM"].contains(&scratch.as_str());
            }

            scratch.clear();
            'pslope: {
                for i in -1isize..=1 {
                    if let Some(x) = center_x.checked_add_signed(-i) {
                        if let Some(y) = center_y.checked_add_signed(i) {
                            if x >= width || y >= height { break 'pslope; }
                            scratch.push(char_at(x, y));
                        }
                    }
                }
                pslope_matched = ["MAS", "SAM"].contains(&scratch.as_str());
            }

            total += (nslope_matched & pslope_matched) as usize;
        }
    }
    println!("total: {total}");
}
