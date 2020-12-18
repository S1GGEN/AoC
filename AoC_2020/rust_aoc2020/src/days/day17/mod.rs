use std::fs;
use std::str;

pub fn load_input(filename: &str) -> String {
    let input = fs::read_to_string(format!("src/days/day17/{}.txt", filename))
        .expect("Something went wrong reading the file");

    return input;
}

pub fn one(input: &str) -> String {
    let result = calculate_one(input);

    match result {
        Some(r) => format!("Task 1: {}", r),
        None => "Task 1: FAILED".to_string(),
    }
}

fn calculate_one(input: &str) -> Option<u64> {
    let initial_state = get_initial_state(input);

    let mut state = initial_state;
    for _ in 0..6 {
        state = iteration(state);
    }

    Some(count_active(state))
}

fn count_active(state : Vec<Vec<Vec<NodeState>>>) -> u64 {
    let mut count = 0;
    for z_dimension in state {
        for x_dimension in z_dimension {
            for node in x_dimension {
                match node {
                    NodeState::ACTIVE => {count += 1}
                    NodeState::INACTIVE => {}
                }
            }
        }
    }

    count
}

fn print_state(state : &Vec<Vec<Vec<NodeState>>>, message : &str) {
    println!("\n{}", message);

    for z_dimension in state {
        println!("\n");
        for x_dimension in z_dimension {
            println!("{:?}", x_dimension);
        }
    }
}

fn iteration(old_state : Vec<Vec<Vec<NodeState>>>) -> Vec<Vec<Vec<NodeState>>> {
    let mut new_state: Vec<Vec<Vec<NodeState>>> = vec![];

    for z in 0..(old_state.len() + 2 ) {
        let mut z_dimension : Vec<Vec<NodeState>> = vec![];
        for x in 0..(old_state[0].len() + 2) {
            let mut x_dimension : Vec<NodeState> = vec![];
            for y in 0..(old_state[0][0].len() + 2) {
                // println!("\nz: {} x: {} y: {}", z, x, y);
                let pos_active = position_is_active(&old_state, z as i32, x as i32, y as i32);
                // println!("pos_active: {}", pos_active);
                let num_neighbours = num_active_neighbours(&old_state, z as i32, x as i32, y as i32);
                // println!("num_neighbours: {}", num_neighbours);


                if pos_active && (num_neighbours == 2 || num_neighbours == 3) {
                    x_dimension.push(NodeState::ACTIVE)
                } else if !pos_active && num_neighbours == 3 {
                    // Yes, these two could be combined, but it would hurt legibility
                    x_dimension.push(NodeState::ACTIVE)
                } else {
                    x_dimension.push(NodeState::INACTIVE)
                }
            }
            // if x_dimension.contains(&NodeState::ACTIVE) {
            z_dimension.push(x_dimension);
            // }
        }
        new_state.push(z_dimension);
    }

    new_state
}

fn num_active_neighbours(old_state : &Vec<Vec<Vec<NodeState>>>, z : i32, x : i32, y : i32) -> u8 {
    let mut number_active : u8 = 0;
    for check_z in (z - 1)..(z + 2) {
        for check_x in (x - 1)..(x + 2) {
            for check_y in (y - 1)..(y + 2) {
                if !(check_z == z && check_x == x && check_y == y) { // A cube is not its own neighbour
                    number_active += position_is_active(&old_state, check_z, check_x, check_y) as u8;
                }
            }
        }
    }

    number_active
}

fn position_is_active(old_state : &Vec<Vec<Vec<NodeState>>>, z : i32, x : i32, y : i32) -> bool {
    return if z < 1 || z > old_state.len() as i32 || x < 1 || x > old_state[0].len() as i32 || y < 1 || y > old_state[0][0].len() as i32 {
        // All outside the old range are definitely inactive
        false
    } else {
        match old_state[z as usize - 1][x as usize - 1][y as usize - 1] {
            NodeState::ACTIVE => { true }
            NodeState::INACTIVE => { false }
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
enum NodeState {
    ACTIVE,
    INACTIVE
}

fn get_initial_state(input : &str) -> Vec<Vec<Vec<NodeState>>> {
    let mut state: Vec<Vec<Vec<NodeState>>> = vec![vec![]];

    for line in input.lines() {
        let mut new_line = vec![];
        for node in line.chars() {
            if node == '.' {
                new_line.push(NodeState::INACTIVE)
            } else {
                new_line.push(NodeState::ACTIVE)
            }
        }
        state[0].push(new_line)
    }

    state
}

pub fn two(input: &str) -> String {
    let result = calculate_two(input);

    match result {
        Some(r) => format!("Task 2: {}", r),
        None => "Task 2: FAILED".to_string(),
    }
}

fn calculate_two(input: &str) -> Option<u64> {
    let initial_state = get_initial_state_4d(input);

    let mut state = initial_state;
    for _ in 0..6 {
        state = iteration_4d(state);
    }

    Some(count_active_4d(state))
}

fn count_active_4d(state : Vec<Vec<Vec<Vec<NodeState>>>>) -> u64 {
    let mut count = 0;
    for z_dimension in state {
        for w_dimension in z_dimension {
            for x_dimension in w_dimension {
                for node in x_dimension {
                    match node {
                        NodeState::ACTIVE => { count += 1 }
                        NodeState::INACTIVE => {}
                    }
                }
            }
        }
    }

    count
}

fn print_state_4d(state : &Vec<Vec<Vec<NodeState>>>, message : &str) {
    println!("\n{}", message);

    for z_dimension in state {
        println!("\n");
        for x_dimension in z_dimension {
            println!("{:?}", x_dimension);
        }
    }
}

fn iteration_4d(old_state : Vec<Vec<Vec<Vec<NodeState>>>>) -> Vec<Vec<Vec<Vec<NodeState>>>> {
    let mut new_state: Vec<Vec<Vec<Vec<NodeState>>>>= vec![];

    for z in 0..(old_state.len() + 2 ) {
        let mut z_dimension : Vec<Vec<Vec<NodeState>>> = vec![];
        for w in 0..(old_state[0].len() + 2) {
            let mut w_dimension : Vec<Vec<NodeState>> = vec![];
            for x in 0..(old_state[0][0].len() + 2) {
                let mut x_dimension: Vec<NodeState> = vec![];
                for y in 0..(old_state[0][0][0].len() + 2) {
                    // println!("\nz: {} x: {} y: {}", z, x, y);
                    let pos_active = position_is_active_4d(&old_state, z as i32, w as i32, x as i32, y as i32);
                    // println!("pos_active: {}", pos_active);
                    let num_neighbours = num_active_neighbours_4d(&old_state, z as i32, w as i32, x as i32, y as i32);
                    // println!("num_neighbours: {}", num_neighbours);


                    if pos_active && (num_neighbours == 2 || num_neighbours == 3) {
                        x_dimension.push(NodeState::ACTIVE)
                    } else if !pos_active && num_neighbours == 3 {
                        // Yes, these two could be combined, but it would hurt legibility
                        x_dimension.push(NodeState::ACTIVE)
                    } else {
                        x_dimension.push(NodeState::INACTIVE)
                    }
                }
                // if x_dimension.contains(&NodeState::ACTIVE) {
                w_dimension.push(x_dimension);
                // }
            }
            z_dimension.push(w_dimension);
        }
        new_state.push(z_dimension);
    }

    new_state
}

fn num_active_neighbours_4d(old_state : &Vec<Vec<Vec<Vec<NodeState>>>>, z : i32, w : i32, x : i32, y : i32) -> u8 {
    let mut number_active : u8 = 0;
    for check_z in (z - 1)..(z + 2) {
        for check_w in (w - 1)..(w + 2) {
            for check_x in (x - 1)..(x + 2) {
                for check_y in (y - 1)..(y + 2) {
                    if !(check_z == z && check_w == w && check_x == x && check_y == y) { // A cube is not its own neighbour
                        number_active += position_is_active_4d(&old_state, check_z, check_w, check_x, check_y) as u8;
                    }
                }
            }
        }
    }

    number_active
}

fn position_is_active_4d(old_state : &Vec<Vec<Vec<Vec<NodeState>>>>, z : i32, w : i32, x : i32, y : i32) -> bool {
    return if
    z < 1 || z > old_state.len() as i32
        || w < 1 || w > old_state[0].len() as i32
        || x < 1 || x > old_state[0][0].len() as i32
        || y < 1 || y > old_state[0][0][0].len() as i32
    {
        // All outside the old range are definitely inactive
        false
    } else {
        match old_state[z as usize - 1][w as usize -1][x as usize - 1][y as usize - 1] {
            NodeState::ACTIVE => { true }
            NodeState::INACTIVE => { false }
        }
    }
}


fn get_initial_state_4d(input : &str) -> Vec<Vec<Vec<Vec<NodeState>>>> {
    let mut state: Vec<Vec<Vec<Vec<NodeState>>>> = vec![vec![vec![]]];

    for line in input.lines() {
        let mut new_line = vec![];
        for node in line.chars() {
            if node == '.' {
                new_line.push(NodeState::INACTIVE)
            } else {
                new_line.push(NodeState::ACTIVE)
            }
        }
        state[0][0].push(new_line)
    }

    state
}