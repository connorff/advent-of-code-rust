use std::{cell::Cell, fs};

type Point = (i32, i32, Cell<i32>);
type Segment = (Point, Point);
type Graph = Vec<Point>;

fn create_point(pts_str: &str) -> Point {
    let mut split = pts_str.split(",");
    (
        split.next().unwrap().parse::<i32>().unwrap(),
        split.next().unwrap().parse::<i32>().unwrap(),
        Cell::new(0),
    )
}

fn find_point(graph: &Graph, x: i32, y: i32) -> Option<&Point> {
    graph.iter().find(|pt| pt.0 == x && pt.1 == y)
}

fn find_dangerous(graph: &Graph) -> Vec<&Point> {
    graph.iter().filter(|point| point.2.get() >= 2).collect()
}

fn populate_graph(graph: &mut Graph, segment: &Segment) {
    let diff_x = segment.0 .0 - segment.1 .0;
    let diff_y = segment.0 .1 - segment.1 .1;

    let subtract_x = diff_x > 0;
    let subtract_y = diff_y > 0;

    let is_diagonal = diff_x.abs() == diff_y.abs();
    let is_vertical = diff_x == 0;
    let is_horizontal = diff_y == 0;

    if !(is_vertical || is_horizontal || is_diagonal) {
        return;
    }

    for num in 0..*[diff_x.abs(), diff_y.abs()].iter().max().unwrap() + 1 {
        let x = if is_horizontal || is_diagonal {
            segment.0 .0 + if subtract_x { -num } else { num }
        } else {
            segment.0 .0
        };

        let y = if is_vertical || is_diagonal {
            segment.0 .1 + if subtract_y { -num } else { num }
        } else {
            segment.0 .1
        };

        let pt_exists = find_point(graph, x, y).is_some();
        if !pt_exists {
            let new_pt = create_point(&format!("{},{}", x, y)[..]);
            graph.push(new_pt);
        }

        let point = find_point(graph, x, y).unwrap();
        point.2.set(point.2.get() + 1);
    }
}

pub fn main() -> usize {
    let data = fs::read_to_string("src/day05/segments.txt").expect("Unable to read file");
    let segments: Vec<Segment> = data
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| {
            let mut pts = line
                .split(" -> ")
                .collect::<Vec<&str>>()
                .into_iter()
                .map(create_point)
                .collect::<Vec<Point>>()
                .into_iter();

            (pts.next().unwrap(), pts.next().unwrap())
        })
        .collect();

    let mut graph: Graph = vec![];
    segments.iter().for_each(|seg| {
        populate_graph(&mut graph, seg);
    });

    find_dangerous(&graph).len()
}
