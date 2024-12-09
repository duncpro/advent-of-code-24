#[derive(Clone, Copy)]
enum Direction { North, South, East, West }

fn step(dir: Direction, curr_x: isize, curr_y: isize) -> (isize, isize)
{
    match dir {
        Direction::North => (curr_x, curr_y - 1),
        Direction::South => (curr_x, curr_y + 1),
        Direction::East => (curr_x + 1, curr_y),
        Direction::West => (curr_x - 1, curr_y),
    }
}

fn next_dir(dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::East => Direction::South,
        Direction::West => Direction::North,
    }
}

const EMPTY_CELL_CH: u8 = '.' as u8;
const POI_CELL_CH: u8 = '#' as u8;
const ORIGIN_CELL_CH: u8 = '^' as u8;

use std::collections::HashSet;

fn main() {
    let mut grid = std::fs::read("input.txt").unwrap();
    let mut visited = HashSet::<(isize, isize)>::new();
    
    let height: isize = grid.iter().copied().filter(|x| *x == '\n' as u8).count()
        .try_into().unwrap();
    let width: isize = grid.iter().copied().take_while(|x| *x != '\n' as u8).count()
        .try_into().unwrap();
    
    let calc_idx = |x: isize, y: isize| usize::try_from(y * (width + 1) + x).unwrap();
    let calc_coord = |idx: isize| (idx % (width + 1), idx / (width + 1));
    
    let origin_idx = grid.iter().position(|x| *x == ORIGIN_CELL_CH).unwrap();
    grid[origin_idx] = EMPTY_CELL_CH;

    let (mut x, mut y) = calc_coord(origin_idx.try_into().unwrap());
    let mut direction = Direction::North;

    loop {
        visited.insert((x, y));
        let (target_x, target_y) = step(direction, x, y);
        if !(0..width).contains(&target_x) || !(0..height).contains(&target_y) {
            break;
        }
        let target_idx = calc_idx(target_x, target_y);
        if grid[target_idx] == POI_CELL_CH {
            direction = next_dir(direction);
            continue;
        }
        assert_eq!(grid[target_idx], EMPTY_CELL_CH);
        x = target_x; y = target_y;
    }

    println!("{}", visited.len());
}
