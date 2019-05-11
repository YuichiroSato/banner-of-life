use cells::*;

pub fn next(cells: &Cells) -> Cells {
    let mut new_cells = Cells::new(cells.size_x, cells.size_y);

    for x in 0..cells.size_x {
        for y in 0..cells.size_y {
            let c = count_alive_around(&cells, x, y);
            if cells.is_alive(x, y) && c == 2 || c == 3 {
                new_cells.make_alive(x, y);
            }
        }
    }

    new_cells
}


fn count_alive_around(cells: &Cells, x: usize, y: usize) -> i32 {
    let mut count = 0;
    let left = (x as i32 - 1) as usize;
    let right = (x as i32 + 1) as usize;
    let upper = (y as i32 - 1) as usize;
    let lower = (y as i32 + 1) as usize;

    if cells.is_alive(left, upper) {
        count += 1;
    }
    if cells.is_alive(x, upper) {
        count += 1;
    }
    if cells.is_alive(right, upper) {
        count += 1;
    }
    if cells.is_alive(left, y) {
        count += 1;
    }
    if cells.is_alive(right, y) {
        count += 1;
    }
    if cells.is_alive(left, lower) {
        count += 1;
    }
    if cells.is_alive(x, lower) {
        count += 1;
    }
    if cells.is_alive(right, lower) {
        count += 1;
    }

    count
}

#[test]
fn test_next() {
    let cells = Cells::from_vec(vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 0],
    ]);

    let expected = Cells::from_vec(vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 0],
    ]);

    assert_eq!(expected, next(&cells));
    assert_eq!(cells, next(&expected));
}

#[test]
fn test_count_alive_around() {
    let mut cells = Cells::new(10, 5);
    cells.make_alive(0, 1);
    cells.make_alive(0, 2);
    cells.make_alive(1, 0);
    cells.make_alive(2, 2);
    cells.make_alive(9, 0);
    cells.make_alive(2, 4);

    assert_eq!(3, count_alive_around(&cells, 0, 0));
    assert_eq!(2, count_alive_around(&cells, 1, 0));
    assert_eq!(2, count_alive_around(&cells, 2, 0));
    assert_eq!(1, count_alive_around(&cells, 3, 0));
    assert_eq!(3, count_alive_around(&cells, 0, 1));
    assert_eq!(4, count_alive_around(&cells, 1, 1));
    assert_eq!(2, count_alive_around(&cells, 2, 1));
    assert_eq!(1, count_alive_around(&cells, 3, 1));
    assert_eq!(1, count_alive_around(&cells, 0, 2));
    assert_eq!(3, count_alive_around(&cells, 1, 2));
    assert_eq!(0, count_alive_around(&cells, 2, 2));
    assert_eq!(1, count_alive_around(&cells, 3, 2));
    assert_eq!(2, count_alive_around(&cells, 0, 4));
    assert_eq!(2, count_alive_around(&cells, 1, 4));
    assert_eq!(1, count_alive_around(&cells, 2, 4));
    assert_eq!(1, count_alive_around(&cells, 3, 4));
}
