fn main() {
    let input = include_str!("./input.txt");
    let grid = parse_input(input);
    println!(
        "Total accessible places (naive): {}",
        accessible_occupied_places(&grid).len()
    );
    println!(
        "Total accessible places (dynamic): {}",
        removeable_places(&grid)
    );
}

// The grid is modeled as a 2d vector of booleans,
// where True indicates a paper roll
fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.trim().chars().map(|c| c == '@').collect())
        .collect()
}

fn print_grid(grid: &[Vec<bool>]) {
    for line in grid {
        for spot in line {
            print!(
                "{}",
                match spot {
                    true => "@",
                    false => ".",
                }
            )
        }
        println!();
    }
    println!();
}

const NEIGHBOUR_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get_occupied_neighbours(grid: &[Vec<bool>], y: usize, x: usize) -> usize {
    let mut occupied_neighbours = 0;
    for (y_off, x_off) in NEIGHBOUR_OFFSETS {
        let y_n = y as isize + y_off;
        let x_n = x as isize + x_off;

        if y_n < 0 || x_n < 0 {
            continue;
        }

        if y_n as usize >= grid.len() {
            continue;
        }

        let row = &grid[y_n as usize];

        if x_n as usize >= row.len() {
            continue;
        }

        if row[x_n as usize] {
            occupied_neighbours += 1;
        }
    }

    occupied_neighbours
}

fn accessible_occupied_places(grid: &[Vec<bool>]) -> Vec<(usize, usize)> {
    let mut accessible_places: Vec<(usize, usize)> = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, spot) in line.iter().enumerate() {
            if *spot && get_occupied_neighbours(grid, y, x) < 4 {
                accessible_places.push((y, x));
            }
        }
    }

    accessible_places
}

fn prune_grid(grid: &[Vec<bool>], accessible_places: &[(usize, usize)]) -> Vec<Vec<bool>> {
    let mut new_grid = grid.to_vec();
    for (y, x) in accessible_places {
        new_grid[*y][*x] = false;
    }

    new_grid
}

fn removable_places_recursive(pruned_grid: &[Vec<bool>], total_removed_count: usize) -> usize {
    let accessible_places = accessible_occupied_places(pruned_grid);
    match accessible_places.len() {
        0 => total_removed_count,
        _ => {
            let pruned_grid = prune_grid(pruned_grid, &accessible_places);
            print_grid(&pruned_grid);
            removable_places_recursive(&pruned_grid, total_removed_count + accessible_places.len())
        }
    }
}

fn removeable_places(grid: &[Vec<bool>]) -> usize {
    print_grid(grid);
    removable_places_recursive(grid, 0)
}
