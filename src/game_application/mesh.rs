use game_application::cgmath::{Vector3, Point3};
use game_application::vertex::Vertex;
use std::collections::{HashMap, HashSet};
use game_application::geometry::normalize;
use std::iter::FromIterator;


type Point = Point3<usize>;


#[derive(Eq, Hash, Copy, Clone)]
struct Triangle {
    points: [Point; 3]
}


impl Triangle {
    pub fn new(points: &[Point]) -> Triangle {
        if points.len() != 3 {
            panic!("Wrong triangle points number: {}", points.len());
        }
        Triangle {
            points: [
                points[0],
                points[1],
                points[2]
            ]
        }
    }

    fn get_normal(&self) -> Vector3<f32> {
        let position1 = self.points[0].cast::<f32>().unwrap();
        let position2 = self.points[1].cast::<f32>().unwrap();
        let position3 = self.points[2].cast::<f32>().unwrap();
        let mut normal = (position2 - position1).cross(position3 - position1);
        normalize(&mut normal);
        normal
    }
}


impl PartialEq for Triangle {
    fn eq(&self, other: &Triangle) -> bool {
        let self_set: HashSet<&Point> = HashSet::from_iter(self.points.iter());
        let other_set: HashSet<&Point> = HashSet::from_iter(other.points.iter());
        self_set == other_set
    }
}


#[derive(Eq, Hash, Copy, Clone)]
struct Edge {
    points: [Point; 2]
}


impl Edge {
    fn new(start: &Point, end: &Point) -> Edge {
        Edge {
            points: [
                *start,
                *end 
            ]
        }
    }
}


impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        let self_set: HashSet<&Point> = HashSet::from_iter(self.points.iter());
        let other_set: HashSet<&Point> = HashSet::from_iter(other.points.iter());
        self_set == other_set
    }
}


pub struct Mesh {
    edge_neighbors: HashMap<Edge, Vec<Triangle>>,
    point_neighbors: HashMap<Point, Vec<Triangle>>
}


impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            edge_neighbors: HashMap::new(),
            point_neighbors: HashMap::new()
        }
    }

    pub fn add_triangle(&mut self, points: &[Point]) {
        let new_triangle = Triangle::new(points); // Если точек не 3, треугольник не создастся
        // Добавляем треугольник соседом всем его вершинам
        for point in points {
            if !self.point_neighbors.contains_key(point) {
                self.point_neighbors.insert(*point, Vec::new());
            }
            self.point_neighbors.get_mut(&point).unwrap().push(new_triangle);
        }
        // Добавляем треугольник соседом всем его ребрам.
        for &(i, j) in [(0, 1), (0, 2), (1, 2)].iter() {
            let edge = Edge::new(&points[i], &points[j]);
            if !self.edge_neighbors.contains_key(&edge) {
                self.edge_neighbors.insert(edge, Vec::new());
            }
            self.edge_neighbors.get_mut(&edge).unwrap().push(new_triangle);
        }
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        let mut result = Vec::new();
        result
    }
}
