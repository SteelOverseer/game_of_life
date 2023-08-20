use leptos::*;

#[derive(Debug, Clone, Copy)]
struct Cell {
    alive: bool
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        // <button
        //     on:click=move |_| {
        //         set_count.update(|n| *n += 1);
        //     }
        //     class:red=move || count.get() % 2 == 1
        // >
        //     "Click me: "
        //     {move || count.get()}
        // </button>
        // <progress max="50" />
        <table>  
        <thead>header</thead>
            <tbody>
            <tr>
                <td>test</td>  
                <td>test 2 </td>          
            </tr>
            </tbody>
        </table>
    }
}

// function to compute the next generation
fn create_next_generation(grid: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let rows = grid.len();
    let cols = grid[0].len();

    // create an empty grid to compute the future generation
    let mut future: Vec<Vec<Cell>> = vec![vec![Cell{alive: false}; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {

            let cell_state = grid[i][j].alive;

            let mut live_neighbors = 0;

            // iterate through every neighbors including the current cell
            for x in -1i8..=1 {
                for y in -1i8..=1 {

                    // position of one of the neighbors (new_x, new_y)
                    let new_x = (i as i8) + x;
                    let new_y = (j as i8) + y;

                    // make sure the position is within the bounds of the grid
                    if new_x > 0 && new_y > 0 && new_x < rows as i8 && new_y < cols as i8 {
                        let neighbor_cell = grid[new_x as usize][new_y as usize];
                        if neighbor_cell.alive {
                            live_neighbors += 1;
                        }
                    }
                }
            }

            // substract the state of the current cell to get the number of alive neighbors
            if cell_state {
                live_neighbors -= 1;
            }

            // applying the rules of game of life to get the future generation
            if cell_state && live_neighbors < 2 { // under population
                future[i][j].alive = false;
            } else if cell_state && (live_neighbors == 2 || live_neighbors == 3) { // live on
                future[i][j].alive = true;
            } else if cell_state && live_neighbors > 3 { // over populations
                future[i][j].alive = false;
            } else if !cell_state && live_neighbors == 3 { // reproduction
                future[i][j].alive = true;
            } else {
                future[i][j].alive = cell_state;
            }
        }
    }

    // return the future generation
    future
}

fn main() {
    let (rows, cols) = (5, 5);

    // create the grid
    let mut grid: Vec<Vec<Cell>> = vec![vec![Cell { alive: false }; cols]; rows];

    // set the initial state of the grid (blinker)
    grid[1][2].alive = true;
    grid[2][2].alive = true;
    grid[3][2].alive = true;

    // print the initial state of the grid;
    println!("Initial grid:");
    grid.iter().for_each(|i| {
        println!("{:?}", i);
    });

    println!("");

    // Number of generations
    const ITR: u8 = 5;

    // compute and print the next generation
    for i in 0..ITR {
        grid = create_next_generation(&grid);

        println!("Generation {}:", i+1);
        grid.iter().for_each(|i| {
            println!("{:?}", i);
        });
        println!("");
    }

    // leptos::mount_to_body(|cx| view! { cx, <App/> })
}