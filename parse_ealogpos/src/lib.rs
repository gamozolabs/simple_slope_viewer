use std::io;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub time:   f64,
    pub x:      f64,
    pub y:      f64,
    pub map_id: u32,
    pub angle:  f64,
}

#[derive(Debug, Clone)]
pub struct Positions {
    pub positions: Vec<Position>,
}

impl Positions {
    pub fn from_lua<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        // Read the LUA saved variable data
        let data = std::fs::read_to_string(path)?;

        // Create the empty position vector
        let mut positions = Vec::new();

        let mut last_time = 0.0;
        for data in data.split("LOCATION237945_").skip(1) {
            let mut data = data.splitn(2, "\",").next().unwrap().split("_");
            let x:      f64 = data.next().unwrap().parse().unwrap();
            let y:      f64 = data.next().unwrap().parse().unwrap();
            let _z:     f64 = data.next().unwrap().parse().unwrap();
            let map_id: u32 = data.next().unwrap().parse().unwrap();
            let angle:  f64 = data.next().unwrap().parse().unwrap();
            let time:   f64 = data.next().unwrap().parse().unwrap();
            assert!(data.next().is_none());

            if last_time == 0. {
                last_time = time;
            }

            if (time - last_time) > 5. {
                print!("Updated {}\n", time);
                print!("{} {} {}\n", x, y, angle);
            }

            last_time = time;

            positions.push(Position {
                time, x, y, map_id, angle
            });
        }

        Ok(Positions {
            positions
        })
    }
}

