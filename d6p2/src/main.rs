// Runtime complexity is O(m * n) where m is the number of steps in
// the original path and n is the number of steps in the candidate paths.
//
// The candidate paths can be heterogenous in terms of length.
// But we can give a loose upperbound on the runtime complexity by assuming 
// n is equal to the size of the world (number of cells) w times the 4 directions. 
// 
// We can also bound m loosely by w, giving us a summary upperbound on the runtime 
// complexity of O(4 * w^2) = O(w^2).

// -- Worldfile ---------------------------------------------------------------

const EMPTY_CELL_CH: u8 = '.' as u8;
const POI_CELL_CH: u8 = '#' as u8;
const ORIGIN_CELL_CH: u8 = '^' as u8;

fn read_worldfile() -> (World, /* origin_x: */ isize, /* origin_y: */ isize) {
    let mut grid = std::fs::read("input.txt").unwrap();
    
    let height: isize = grid.iter().copied().filter(|x| *x == '\n' as u8).count()
        .try_into().unwrap();
    let width: isize = grid.iter().copied().take_while(|x| *x != '\n' as u8).count()
        .try_into().unwrap();
    
    let calc_coord = |idx: isize| (idx % (width + 1), idx / (width + 1));
    
    let origin_idx = grid.iter().position(|x| *x == ORIGIN_CELL_CH).unwrap();
    grid[origin_idx] = EMPTY_CELL_CH;

    let (origin_x, origin_y) = calc_coord(origin_idx.try_into().unwrap());

    let world = World { width, height, grid };

    return (world, origin_x, origin_y);
}

struct World { width: isize, height: isize, grid: Vec<u8> }

impl<'a> World {
    fn calc_idx(&self, x: isize, y: isize) -> usize {
        (y * (self.width + 1) + x).try_into().unwrap()
    }

    fn get(&self, x: isize, y: isize) -> &u8 {
        &self.grid[self.calc_idx(x, y)]
    }

    fn get_mut(&mut self, x: isize, y: isize) -> &mut u8 {
        let idx = self.calc_idx(x, y);
        &mut self.grid[idx]
    }
}

// -- Direction -------------------------------------------------------------

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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

// -- walk -----------------------------------------------------------------

fn walk<V>(world: &World, origin_x: isize, origin_y: isize, mut visit: V) 
where V: FnMut(isize, isize, Direction) -> bool /* !break */
{

    let (mut x, mut y) = (origin_x, origin_y);
    let mut direction = Direction::North;
    
    loop {
        if !(visit)(x, y, direction) { break; }
        
        let (target_x, target_y) = step(direction, x, y);
        
        let target_inbounds = (0..(world.width)).contains(&target_x)
            && (0..(world.height)).contains(&target_y);
        if !target_inbounds { break; }
        
        let target_occ = *world.get(target_x, target_y);
        
        if target_occ == POI_CELL_CH {
            direction = next_dir(direction);
            continue;
        }
        
        assert_eq!(target_occ, EMPTY_CELL_CH);
        x = target_x; y = target_y;
    }
}

// -- main ----------------------------------------------------------------

fn main() {
    let (mut world, origin_x, origin_y) = read_worldfile();

    let mut candidates = std::collections::HashSet::<(isize, isize)>::new();

    walk(&world, origin_x, origin_y, |x, y, _| { candidates.insert((x, y)); true });

    assert!(candidates.remove(&(origin_x, origin_y)));

    let mut canididates_list = Vec::from_iter(candidates.iter().copied());
    let mut total_accepted = 0usize;

    while let Some(candidate) = canididates_list.pop() {
        let (candidate_x, candidate_y) = candidate;

        *world.get_mut(candidate_x, candidate_y) = POI_CELL_CH;

        let mut visited = std::collections::HashSet::<(isize, isize, Direction)>::new();
        let mut is_cycle = false;
        
        walk(&world, origin_x, origin_y, |x, y, dir| {
            let is_fresh = visited.insert((x, y, dir));
            is_cycle = !is_fresh;
            return is_fresh; // continue walking while fresh
        });

        *world.get_mut(candidate_x, candidate_y) = EMPTY_CELL_CH;

        total_accepted += is_cycle as usize;
    }
    
    println!("{total_accepted}");
}

