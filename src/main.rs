use std::{thread, time};

use leptos::{*, html::Input, ev::SubmitEvent, svg::view, leptos_dom::console_log};

#[derive(Debug, Clone, Copy)]
struct Cell {
    alive: bool
}

#[component]
fn App(cx: Scope) -> impl IntoView {
	let (rows, set_rows) = create_signal(cx, 5);
	let (columns, set_columns) = create_signal(cx, 5);
	
	// create the grid
	let (grid, set_grid) = create_signal(cx, vec![vec![Cell { alive: false }; columns.get()]; rows.get()]);
	
	// set the initial state of the grid (blinker)
	set_grid.update(|grid|{
		grid[1][2].alive = true
	});
	set_grid.update(|grid|{
		grid[2][2].alive = true
	});
	set_grid.update(|grid|{
		grid[3][2].alive = true
	});

	view! { cx,
		<Grid grid=grid />
	}
}

#[component]
fn Grid(cx: Scope, grid: ReadSignal<Vec<Vec<Cell>>>) -> impl IntoView {
    let rows = grid.get().len();
    let columns = grid.get()[0].len();
    view! { cx,
        {(0..rows)
            .into_iter()
            .map(|x| {
                view! { cx,
                    <div class="row">
                        {(0..columns)
                            .into_iter()
                            .map(|y| {
                                view! { cx, <div class="cell" class:alive = grid.get()[x][y].alive></div> }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                }
            })
            .collect::<Vec<_>>()
		}
	}
}

// function to compute the next generation
fn create_next_generation(cx: Scope, grid: &ReadSignal<Vec<Vec<Cell>>>) -> ReadSignal<Vec<Vec<Cell>>> {
    let rows = grid.get().len();
    let cols = grid.get()[0].len();
    
    // create an empty grid to compute the future generation
	let (future_grid, set_grid) = create_signal(cx, vec![vec![Cell { alive: false }; cols]; rows]);

    for i in 0..rows {
        for j in 0..cols {

            let cell_state = grid.get()[i][j].alive;

            let mut live_neighbors = 0;

            // iterate through every neighbors including the current cell
            for x in -1i8..=1 {
                for y in -1i8..=1 {

                    // position of one of the neighbors (new_x, new_y)
                    let new_x = (i as i8) + x;
                    let new_y = (j as i8) + y;

                    // make sure the position is within the bounds of the grid
                    if new_x > 0 && new_y > 0 && new_x < rows as i8 && new_y < cols as i8 {
                        let neighbor_cell = grid.get()[new_x as usize][new_y as usize];
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
				set_grid.update(|grid|{
					grid[i][j].alive = false;
				});
            } else if cell_state && (live_neighbors == 2 || live_neighbors == 3) { // live on
				set_grid.update(|grid|{
					grid[i][j].alive = true;
				});
            } else if cell_state && live_neighbors > 3 { // over populations
				set_grid.update(|grid|{
					grid[i][j].alive = false;
				});
            } else if !cell_state && live_neighbors == 3 { // reproduction
				set_grid.update(|grid|{
					grid[i][j].alive = true;
				});
            } else {
				set_grid.update(|grid|{
					grid[i][j].alive = cell_state;
				});
            }
        }
    }

    // return the future generation
    return future_grid;
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App /> })
}