use std::collections::HashSet;

use petgraph::algo::astar;
use petgraph::visit::{GraphBase, GraphRef, IntoEdges, IntoNeighbors, Visitable};


pub struct Lattice {}

impl Visitable for &Lattice {
    type Map = HashSet<Point3D>;

    fn visit_map(self: &Self) -> Self::Map {
        HashSet::new()
    }
    fn reset_map(self: &Self, map: &mut Self::Map) {
        map.clear();
    }
}
impl IntoEdges for &Lattice {
    type Edges = ;
}

impl IntoNeighbors for &Lattice {
    type Neighbors = std::vec::IntoIter<Point3D>;
    fn neighbors(self, a: Self::NodeId) -> Self::Neighbors {
        let mut neighbors = Vec::new();

        for dir in Direction3D::all() {
            let nx = a.x + dir.delta.x;
            let ny = a.y + dir.delta.y;
            let nz = a.z + dir.delta.z;

            neighbors.push(Point3D {
                x: nx,
                y: ny,
                z: nz,
            });
        }

        neighbors.into_iter()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Point3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Direction3D {
    pub delta: Point3D,
}

pub struct Direction3DIter {
    dx: i32,
    dy: i32,
    dz: i32,
}

impl Direction3D {
    /// Returns an iterator over all 26 possible moves in 3D (excluding (0,0,0))
    pub fn all() -> Direction3DIter {
        Direction3DIter {
            dx: -1,
            dy: -1,
            dz: -1,
        }
    }
}

impl Iterator for Direction3DIter {
    type Item = Direction3D;

    fn next(&mut self) -> Option<Self::Item> {
        while self.dz <= 1 {
            while self.dy <= 1 {
                while self.dx <= 1 {
                    let dx = self.dx;
                    let dy = self.dy;
                    let dz = self.dz;
                    self.dx += 1;

                    if dx != 0 || dy != 0 || dz != 0 {
                        return Some(Direction3D {
                            delta: Point3D {
                                x: dx,
                                y: dy,
                                z: dz,
                            },
                        });
                    }
                }
                self.dx = -1;
                self.dy += 1;
            }
            self.dy = -1;
            self.dz += 1;
        }
        None
    }
}

impl GraphBase for Lattice {
    type NodeId = Point3D;
    type EdgeId = Direction3D; // unused
}

fn main() {
   /* let lattice = Lattice {};
   let start = Point3D { x: 0, y: 0, z: 0 };
   let goal = Point3D { x: 4, y: 4, z: 4 };

   let result = astar(
       &lattice,
       start,
       |p| *p == goal,
       |_| 1, // uniform edge cost
       |p| (goal.x - p.x).abs() + (goal.y - p.y).abs() + (goal.z - p.z).abs(), // Manhattan distance
   );

   match result {
       Some((cost, path)) => {
           println!("Path found with cost {}:", cost);
           for p in path {
               println!("{:?}", p);
           }
       }
       None => println!("No path found."),
   } */
}
