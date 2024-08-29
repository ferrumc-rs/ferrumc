use std::collections::HashSet;
use rand::Rng;
use tokio::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

struct Octree {
    center: Position,
    half_dimension: i32,
    positions: HashSet<Position>,
    children: Option<Box<[Octree; 8]>>,
}

impl Octree {
    fn new(center: Position, half_dimension: i32) -> Self {
        Octree {
            center,
            half_dimension,
            positions: HashSet::new(),
            children: None,
        }
    }

    fn insert(&mut self, position: Position) {
        if self.half_dimension <= 1000 {  // Increased threshold for subdivision
            self.positions.insert(position);
            return;
        }

        let octant = self.get_octant(position);
        if self.children.is_none() {
            self.subdivide();
        }

        if let Some(ref mut children) = self.children {
            children[octant].insert(position);
        }
    }

    fn subdivide(&mut self) {
        let half = self.half_dimension / 2;
        let x = self.center.x;
        let y = self.center.y;
        let z = self.center.z;

        self.children = Some(Box::new([
            Octree::new(Position { x: x - half, y: y - half, z: z - half }, half),
            Octree::new(Position { x: x + half, y: y - half, z: z - half }, half),
            Octree::new(Position { x: x - half, y: y + half, z: z - half }, half),
            Octree::new(Position { x: x + half, y: y + half, z: z - half }, half),
            Octree::new(Position { x: x - half, y: y - half, z: z + half }, half),
            Octree::new(Position { x: x + half, y: y - half, z: z + half }, half),
            Octree::new(Position { x: x - half, y: y + half, z: z + half }, half),
            Octree::new(Position { x: x + half, y: y + half, z: z + half }, half),
        ]));
    }

    fn get_octant(&self, position: Position) -> usize {
        let mut octant = 0;
        if position.x >= self.center.x { octant |= 1; }
        if position.y as i32 >= self.center.y as i32 { octant |= 2; }
        if position.z >= self.center.z { octant |= 4; }
        octant
    }

    fn query(&self, position: Position, radius: f32) -> Vec<Position> {
        let mut result = Vec::new();
        self.query_recursive(position, radius, &mut result);
        result
    }

    fn query_recursive(&self, position: Position, radius: f32, result: &mut Vec<Position>) {
        if !self.intersects_sphere(position, radius) {
            return;
        }

        for pos in &self.positions {
            if self.distance_squared(*pos, position) <= (radius * radius) as i64 {
                result.push(*pos);
            }
        }

        if let Some(ref children) = self.children {
            for child in children.iter() {
                child.query_recursive(position, radius, result);
            }
        }
    }

    fn intersects_sphere(&self, position: Position, radius: f32) -> bool {
        let dx = (position.x - self.center.x).abs() as f32;
        let dy = (position.y as i32 - self.center.y as i32).abs() as f32;
        let dz = (position.z - self.center.z).abs() as f32;

        let max_dist = self.half_dimension as f32 + radius;

        dx <= max_dist && dy <= max_dist && dz <= max_dist
    }

    fn distance_squared(&self, a: Position, b: Position) -> i64 {
        let dx = a.x as i64 - b.x as i64;
        let dy = a.y as i64 - b.y as i64;
        let dz = a.z as i64 - b.z as i64;
        dx * dx + dy * dy + dz * dz
    }
}
pub async fn test_spatial_query() {
    let center = Position { x: 0, y: 0, z: 0 };
    let mut octree = Octree::new(center, 3_000_000);  // Increased size to cover -3M to +3M range

    let mut rng = rand::thread_rng();

    println!("Inserting 5 million entities...");
    let insert_start = Instant::now();
    const RANGE: i32 = 3_000;
    for _ in 0..5_000_000 {
        let x = rng.gen_range(-RANGE..=RANGE);
        let y = rng.gen_range(-RANGE..=RANGE);
        let z = rng.gen_range(-RANGE..=RANGE);
        octree.insert(Position { x, y, z });
    }
    let insert_duration = insert_start.elapsed();
    println!("Insertion completed in {:?}", insert_duration);

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    println!("Querying for positions within radius {RANGE}...");
    let query_pos = Position { x: 0, y: 0, z: 0 };
    let radius = 300.0;
    let query_start = Instant::now();
    let result = octree.query(query_pos, radius);
    let query_duration = query_start.elapsed();

    println!("Query completed in {:?}", query_duration);
    println!("Found {} positions within radius {} of {:?}", result.len(), radius, query_pos);

    // Print first 10 results as a sample
    for pos in result.iter().take(10) {
        println!("{:?}", pos);
    }
}
