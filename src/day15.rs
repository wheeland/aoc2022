use crate::vec2::Vec2i32;

struct Sensor {
    sensor: Vec2i32,
    radius: i32,
}

struct Spans(Vec<(i32, i32)>);

impl Spans {
    fn add(&mut self, from: i32, to: i32) {
        assert!(to > from);
        self.0.push((from, to));
        self.0.sort_by(|a, b| {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });

        let mut i = 0;
        while i + 1 < self.0.len() {
            if self.0[i].1 >= self.0[i + 1].0 {
                self.0[i].1 = self.0[i].1.max(self.0[i + 1].1);
                self.0.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    fn contains(&self, num: i32) -> bool {
        self.0.iter().any(|span| span.0 <= num && span.1 > num)
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

pub fn solve() {
    let data = include_str!("inputs/15.txt");

    let mut beacons = Vec::new();
    let sensors: Vec<Sensor> = data
        .split("\n")
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.trim().split('=');
            parts.next();
            let sx = parts.next().unwrap();
            let sy = parts.next().unwrap();
            let bx = parts.next().unwrap();
            let by = parts.next().unwrap();
            let sx = sx.split(",").next().unwrap();
            let sy = sy.split(":").next().unwrap();
            let bx = bx.split(",").next().unwrap();
            let sx = sx.parse().unwrap();
            let sy = sy.parse().unwrap();
            let bx = bx.parse().unwrap();
            let by = by.parse().unwrap();

            let sensor = Vec2i32::new(sx, sy);
            let beacon = Vec2i32::new(bx, by);
            let dist = (beacon - sensor).abs();
            let radius = dist.x + dist.y;
            if !beacons.contains(&beacon) {
                beacons.push(beacon);
            }

            Sensor { sensor, radius }
        })
        .collect();

    let mut spans = Spans(vec![]);
    {
        let y = 2000000;

        for sensor in &sensors {
            let reach = sensor.radius - (sensor.sensor.y - y).abs();
            if reach >= 0 {
                spans.add(sensor.sensor.x - reach, sensor.sensor.x + reach + 1);
            }
        }

        let mut task1 = spans.0.iter().map(|span| span.1 - span.0).sum::<i32>();
        for beacon in beacons {
            if beacon.y == y && spans.contains(beacon.x) {
                task1 -= 1;
            }
        }
        println!("[day 15] task 1: {}", task1);
    }

    let sz = 4000000;
    // let mut task2 = (0, 0);
    // {
    //     for y in 0..sz + 1 {
    //         spans.clear();
    //         for sensor in &sensors {
    //             let reach = sensor.radius - (sensor.sensor.y - y).abs();
    //             if reach >= 0 {
    //                 spans.add(sensor.sensor.x - reach, sensor.sensor.x + reach + 1);
    //             }
    //         }
    //         if spans.0.len() > 1 {
    //             task2 = (spans.0[0].1, y);
    //             break;
    //         } else if spans.0.first().unwrap().0 > 0 {
    //             task2 = (0, y);
    //             break;
    //         } else if spans.0.last().unwrap().1 < sz {
    //             task2 = (sz, y);
    //         }
    //     }
    // }
    // println!(
    //     "[day 15] task 2: {}",
    //     task2.0 as i64 * sz as i64 + task2.1 as i64
    // );
}
