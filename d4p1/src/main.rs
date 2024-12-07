// In hindsight the code would have been shorter and easier to read if I dispensed with the
// iterators and did it procedurally with a scratch buffer.
//
// In the future I might come back to this and rewrite it procedurally with the same
// peformance characteristics, but better readability.
//
// Or, I could stay with the iterators, and then implement a .matches function for char iterators
// which would bring the space complexity down to O(1)! It is **definitely** possible, but might
// not be worth the time investment.

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    
    let read_char = |x: usize, y: usize| input.as_bytes()[y * (width + 1) + x] as char;

    let mut total = 0usize;

    // scan negative-slope diagonals
    {
        let neg_diag_roots = std::iter::Iterator::chain(
            (0..height).map(|y| (0, y)),
            (1..width).map(|x| (x, 0))
        );
        
        for (init_x, init_y) in neg_diag_roots {
            let diag = (0..)
                .map(|offset| (init_x + offset, init_y + offset))
                .take_while(|(x, y)| *x < width && *y < height)
                .map(|(x, y)| read_char(x, y))
                .collect::<String>();

            total += diag.matches("XMAS").count();
            total += diag.matches("SAMX").count();
        }
    }
    
    // scan positive-slope diagonals
    {
        let pos_diag_roots = std::iter::Iterator::chain(
            (0..width).map(|x| (x, height - 1)),
            (0..(height - 1)).map(|y| (0, y))
        );
        
        for (init_x, init_y) in pos_diag_roots {
            let diag = (0..)
                .map(|offset| (offset, init_y.checked_sub(offset)))
                .take_while(|(_, y)| y.is_some())
                .map(|(offset, y)| (init_x + offset, y.unwrap()))
                .take_while(|(x, _)| *x < width)
                .map(|(x, y)| read_char(x, y))
                .collect::<String>();

            total += diag.matches("XMAS").count();
            total += diag.matches("SAMX").count();
        }
    }

    // scan rows
    for row in input.lines() {
        total += row.matches("XMAS").count();
        total += row.matches("SAMX").count();
    }

    // scan columns
    for x in 0..width {
        let col = (0..height)
            .map(|y| read_char(x, y))
            .collect::<String>();
        total += col.matches("XMAS").count();
        total += col.matches("SAMX").count();
    }
    
    println!("total: {}", total);
}

