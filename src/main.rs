/// dimensions given in the requirements
const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 100;

/// A placement of a tetromino shape in the grid. Each placement consists
/// of 4 squares. Each square is represented as a tuple (x, y) denotes the
/// coordinates on the grid.
struct Placement([(usize, usize); 4]);

/// A tetris block
trait Tetromino {
    /// Returns the width of the tetromino.
    fn width() -> usize;

    /// Returns the height of the tetromino.
    fn height() -> usize;

    /// Returns the placement of a tetromino shape from a given starting
    /// point. The starting point is the bottom-left most square occupied
    /// by the smallest rectangular box that can contain the shape.
    fn placement_at(at: (usize, usize)) -> Result<Placement, &'static str>;
}

struct Q;

impl Tetromino for Q {
    fn width() -> usize {
        2
    }

    fn height() -> usize {
        2
    }

    /// The placement layout for Q is
    ///
    /// 3 4
    /// 1 2
    fn placement_at((x, y): (usize, usize)) -> Result<Placement, &'static str> {
        if x > (GRID_WIDTH - Self::width()) || y > (GRID_HEIGHT - Self::height()) {
            Err("out of bound")
        } else {
            Ok(Placement([(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)]))
        }
    }
}

struct Z;

impl Tetromino for Z {
    fn width() -> usize {
        3
    }

    fn height() -> usize {
        2
    }

    /// The placement layout for Z is
    ///
    /// 3 4
    ///   1 2
    fn placement_at((x, y): (usize, usize)) -> Result<Placement, &'static str> {
        if x > (GRID_WIDTH - Self::width()) || y > (GRID_HEIGHT - Self::height()) {
            Err("out of bound")
        } else {
            Ok(Placement([
                (x + 1, y),
                (x + 2, y),
                (x, y + 1),
                (x + 1, y + 1),
            ]))
        }
    }
}

struct S;

impl Tetromino for S {
    fn width() -> usize {
        3
    }

    fn height() -> usize {
        2
    }

    /// The placement layout for S is
    ///
    ///   3 4
    /// 1 2
    fn placement_at((x, y): (usize, usize)) -> Result<Placement, &'static str> {
        if x > (GRID_WIDTH - Self::width()) || y > (GRID_HEIGHT - Self::height()) {
            Err("out of bound")
        } else {
            Ok(Placement([
                (x, y),
                (x + 1, y),
                (x + 1, y + 1),
                (x + 2, y + 1),
            ]))
        }
    }
}

struct T;

impl Tetromino for T {
    fn width() -> usize {
        3
    }

    fn height() -> usize {
        2
    }

    /// The placement layout for T is
    ///
    /// 2 3 4
    ///   1
    fn placement_at((x, y): (usize, usize)) -> Result<Placement, &'static str> {
        if x > (GRID_WIDTH - Self::width()) || y > (GRID_HEIGHT - Self::height()) {
            Err("out of bound")
        } else {
            Ok(Placement([
                (x + 1, y),
                (x, y + 1),
                (x + 1, y + 1),
                (x + 2, y + 1),
            ]))
        }
    }
}

struct I;

impl Tetromino for I {
    fn width() -> usize {
        4
    }

    fn height() -> usize {
        1
    }

    /// The placement layout for I is
    ///
    /// 1 2 3 4
    fn placement_at((x, y): (usize, usize)) -> Result<Placement, &'static str> {
        if x > (GRID_WIDTH - Self::width()) || y > (GRID_HEIGHT - Self::height()) {
            Err("out of bound")
        } else {
            Ok(Placement([(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]))
        }
    }
}

struct L;

impl Tetromino for L {
    fn width() -> usize {
        2
    }

    fn height() -> usize {
        3
    }

    /// The placement layout for L is
    ///
    /// 4
    /// 3
    /// 1 2
    fn placement_at((x, y): (usize, usize)) -> Result<Placement, &'static str> {
        if x > (GRID_WIDTH - Self::width()) || y > (GRID_HEIGHT - Self::height()) {
            Err("out of bound")
        } else {
            Ok(Placement([(x, y), (x + 1, y), (x, y + 1), (x, y + 2)]))
        }
    }
}

struct J;

impl Tetromino for J {
    fn width() -> usize {
        2
    }

    fn height() -> usize {
        3
    }

    /// The placement layout for J is
    ///
    ///   4
    ///   3
    /// 1 2
    fn placement_at((x, y): (usize, usize)) -> Result<Placement, &'static str> {
        if x > (GRID_WIDTH - Self::width()) || y > (GRID_HEIGHT - Self::height()) {
            Err("out of bound")
        } else {
            Ok(Placement([
                (x, y),
                (x + 1, y),
                (x + 1, y + 1),
                (x + 1, y + 2),
            ]))
        }
    }
}

/// The grid is represented by a 10x100 2 dimensional array of booleans.
/// A `false` indicates that the square at the coordinates is empty.
struct Grid {
    squares: Vec<Vec<bool>>,
    first_blank: usize,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            squares: vec![vec![false; 10]; 100],
            first_blank: 0,
        }
    }

    /// Returns whether a row is completely filled.
    fn is_row_filled(&self, y: usize) -> bool {
        self.squares[y].iter().all(|x| *x)
    }

    /// Returns whether a placement can be placed on the grid, i.e. all the
    /// squares make up the placement are empty.
    fn can_place(&self, p: &Placement) -> bool {
        p.0.iter().all(|(x, y)| !self.squares[*y][*x])
    }

    /// Places the tetromino on the grid.
    pub fn place<T: Tetromino>(&mut self, column: usize) -> Result<(), &'static str> {
        // the first blank row is the obvious candidate. so we will start
        // with a placement there.
        let mut placement = T::placement_at((column, self.first_blank))?;

        // but let's see if we can do better. iterate partially filled rows
        // from top to bottom to check if the tetromino can fit.
        for row in (0..self.first_blank).rev() {
            let p = T::placement_at((column, row))?;
            if self.can_place(&p) {
                placement = p;
            } else {
                // rest of the rows are blocked.
                break;
            }
        }

        // place the tetromino
        placement
            .0
            .iter()
            .for_each(|(x, y)| self.squares[*y][*x] = true);

        // the top of the new placement might be the new height. and the
        // placement is ordered so that the last square has the greatest
        // height
        let top = placement.0[3].1;

        // the first blank row is the greater of the current height and
        // the new height.
        self.first_blank = std::cmp::max(self.first_blank, top + 1);

        // remove the fully filled rows, the max # of rows that can be
        // filled equals to the height of the tetromino.
        for i in 0..T::height() {
            let y = top - i;
            if self.is_row_filled(y) {
                self.squares.remove(y);
                self.squares.push(vec![false; 10]);
                self.first_blank -= 1;
            }
        }

        Ok(())
    }

    /// Returns the height of the remaining blocks.
    pub fn height(&self) -> usize {
        self.first_blank
    }
}

fn solve(input: &str) -> Result<usize, &'static str> {
    let mut grid = Grid::new();

    for entry in input.split(',') {
        let mut chars = entry.trim().chars();
        let tetromino = chars.next().ok_or("missing tetromino")?;
        let column = chars
            .next()
            .map(|c| c.to_digit(10))
            .flatten()
            .map(|i| i as usize)
            .ok_or("bad column")?;

        match tetromino {
            'Q' => grid.place::<Q>(column),
            'Z' => grid.place::<Z>(column),
            'S' => grid.place::<S>(column),
            'T' => grid.place::<T>(column),
            'I' => grid.place::<I>(column),
            'L' => grid.place::<L>(column),
            'J' => grid.place::<J>(column),
            _ => Err("bad tetromino"),
        }?;
    }

    Ok(grid.height())
}

fn main() -> Result<(), &'static str> {
    let stdin = std::io::stdin();

    loop {
        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0 | 1) => break Ok(()),
            Ok(_) => println!("{}", solve(&input)?),
            Err(_) => break Err("crash on fatal error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() -> Result<(), &'static str> {
        assert_eq!(2, solve("Q0")?);
        assert_eq!(4, solve("Q0,Q1")?);
        assert_eq!(0, solve("Q0,Q2,Q4,Q6,Q8")?);
        assert_eq!(2, solve("Q0,Q2,Q4,Q6,Q8,Q1")?);
        assert_eq!(4, solve("Q0,Q2,Q4,Q6,Q8,Q1,Q1")?);
        assert_eq!(1, solve("I0,I4,Q8")?);
        assert_eq!(0, solve("I0,I4,Q8,I0,I4")?);
        assert_eq!(2, solve("L0,J2,L4,J6,Q8")?);
        assert_eq!(2, solve("L0,Z1,Z3,Z5,Z7")?);
        assert_eq!(2, solve("T0,T3")?);
        assert_eq!(1, solve("T0,T3,I6,I6")?);
        assert_eq!(1, solve("I0,I6,S4")?);
        assert_eq!(4, solve("T1,Z3,I4")?);
        assert_eq!(3, solve("L0,J3,L5,J8,T1")?);
        assert_eq!(1, solve("L0,J3,L5,J8,T1,T6")?);
        assert_eq!(2, solve("L0,J3,L5,J8,T1,T6,J2,L6,T0,T7")?);
        assert_eq!(1, solve("L0,J3,L5,J8,T1,T6,J2,L6,T0,T7,Q4")?);
        assert_eq!(8, solve("S0,S2,S4,S6")?);
        assert_eq!(8, solve("S0,S2,S4,S5,Q8,Q8,Q8,Q8,T1,Q1,I0,Q4")?);
        assert_eq!(0, solve("L0,J3,L5,J8,T1,T6,S2,Z5,T0,T7")?);
        assert_eq!(3, solve("Q0,I2,I6,I0,I6,I6,Q2,Q4")?);

        Ok(())
    }

    // check to make sure once a row is cleared, next tetromino can fill
    // in the previously unreachable hole.
    //
    // after I0, I6, T4, J8, T6, the grid layout is
    //
    //                   x
    //             x x x x
    //         x x x x x x
    // x x x x   x x x x x
    //
    // I0 clears out row 1, allowing T3 to fit in the hole in row 0.
    #[test]
    fn extras() -> Result<(), &'static str> {
        assert_eq!(2, solve("I0,I6,T4,J8,T6,I0,T3")?);

        Ok(())
    }
}
