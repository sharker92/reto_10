extern crate piston_window;

use piston_window::*;

impl Grid {
    /// Draws the grid.
    pub fn draw<G>(&self, line: &Line, draw_state: &DrawState, transform: Matrix2d, g: &mut G)
        where G: Graphics
    {
        let &Grid { cols, rows, units } = self;
        for x in 0..cols + 1 {
            let x1 = x as Scalar * units;
            let y1 = 0.0;
            let x2 = x1;
            let y2 = rows as Scalar * units;
            line.draw([x1, y1, x2, y2], draw_state, transform, g);
        }
        for y in 0..rows + 1 {
            let x1 = 0.0;
            let y1 = y as Scalar * units;
            let x2 = cols as Scalar * units;
            let y2 = y1;
            line.draw([x1, y1, x2, y2], draw_state, transform, g);
        }
    }

    /// Get a GridIterator for the grid
    pub fn cells(&self) -> GridCells {
        GridCells {
            cols: self.cols,
            rows: self.rows,
            state: 0,
        }
    }

    /// Get on-screen position of a grid cell
    pub fn cell_position(&self, cell: (u32, u32)) -> Vec2d {
        [cell.0 as Scalar * &self.units, cell.1 as Scalar * &self.units]
    }

    /// Get on-screen x position of a grid cell
    pub fn x_pos(&self, cell: (u32, u32)) -> Scalar {
        self.cell_position(cell)[0]
    }

    /// Get on-screen y position of a grid cell
    pub fn y_pos(&self, cell: (u32, u32)) -> Scalar {
        self.cell_position(cell)[1]
    }
}
///////////////////////////////
pub struct Grid {
    pub cols: u32,
    pub rows: u32,
    pub units: i64,
}
struct Punto {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64
}

fn main() {
    let datos = vec![
Punto { x: 9, y: 1, vx: 0, vy: 2},
Punto { x: 7, y: 0, vx:-1, vy: 0},
Punto { x: 3, y:-2, vx:-1, vy: 1},
Punto { x: 6, y:10, vx:-2, vy:-1},
Punto { x: 2, y:-4, vx: 2, vy: 2},
Punto { x:-6, y:10, vx: 2, vy:-2},
Punto { x: 1, y: 8, vx: 1, vy:-1},
Punto { x: 1, y: 7, vx: 1, vy: 0},
Punto { x:-3, y:11, vx: 1, vy:-2},
Punto { x: 7, y: 6, vx:-1, vy:-1},
Punto { x:-2, y: 3, vx: 1, vy: 0},
Punto { x:-4, y: 3, vx: 2, vy: 0},
Punto { x:10, y:-3, vx:-1, vy: 1},
Punto { x: 5, y:11, vx: 1, vy:-2},
Punto { x: 4, y: 7, vx: 0, vy:-1},
Punto { x: 8, y:-2, vx: 0, vy: 1},
Punto { x:15, y: 0, vx:-2, vy: 0},
Punto { x: 1, y: 6, vx: 1, vy: 0},
Punto { x: 8, y: 9, vx: 0, vy:-1},
Punto { x: 3, y: 3, vx:-1, vy: 1},
Punto { x: 0, y: 5, vx: 0, vy:-1},
Punto { x:-2, y: 2, vx: 2, vy: 0},
Punto { x: 5, y:-2, vx: 1, vy: 2},
Punto { x: 1, y: 4, vx: 2, vy: 1},
Punto { x:-2, y: 7, vx: 2, vy:-2},
Punto { x: 3, y: 6, vx:-1, vy:-1},
Punto { x: 5, y: 0, vx: 1, vy: 0},
Punto { x:-6, y: 0, vx: 2, vy: 0},
Punto { x: 5, y: 9, vx: 1, vy:-2},
Punto { x:14, y: 7, vx:-2, vy: 0},
Punto { x:-3, y: 6, vx: 2, vy:-1}];

let mut state =  vec![vec![0; 1000]; 5];//cols rows
//state[0][1] = 42;
    let malla = Grid{ cols: 10, rows: 5, units: 1};

    //let origin = Punto { x: 0, y: 0 }; // origin: Point

    //println!("The origin is at ({}, {})", origin.x, origin.y);
    println!("El punto 2 is at ({}, {}, {}, {})", datos[1].x, datos[1].y, datos[1].vx, datos[1].vy);
    //malla.draw();
    //println!("{}", malla);
/*    for cols in state.iter(){
        for rows in cols.iter(){
            print!("{}", rows);
        }
        println!();
    }*/
    let mut window: PistonWindow =
        WindowSettings::new("Advent Code 10!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}
