use std::io;
use std::io::BufRead;
use std::collections::{HashSet, HashMap, VecDeque};

fn main() {
    let input = read_stdin();
    let output = process_part_one(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let volcano = Volcano::parse(&input);

    format!("{}", volcano.find_path())
}

fn process_part_two(input: Vec<String>) -> String {
    format!("Output")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

#[derive(Debug, PartialEq)]
struct DistanceMap {
    distances: HashMap<(String, String), isize>
}

impl DistanceMap {
    fn build(valves: &HashMap<String, Valve>) -> Self {

        let mut working_valves = valves
            .iter()
            .filter(|(k, v)| v.flow_rate != 0)
            .map(|(k, v)| k.to_string())
            .collect::<Vec<String>>();

        let mut valve_pairs: Vec<(String, String)> = vec![];

        loop {
            let v_id = working_valves.pop().unwrap();

            for ov_id in working_valves.iter() {
                valve_pairs.push((v_id.to_string(), ov_id.to_string()));
            }

            if working_valves.len() == 1 { break; }
        }

        let distances = valve_pairs
            .iter()
            .filter_map(|(to, from)| {
                let dist = Self::find_shortest_path_distance(valves, from, to);

                dist.map(|d| ((from.to_string(), to.to_string()), d))
            })
            .collect::<HashMap<(String, String), isize>>();

        DistanceMap { distances }
    }

    fn find_shortest_path_distance(
        valves: &HashMap<String, Valve>,
        from: &str,
        to: &str,
    ) -> Option<isize> {
        let mut queue = VecDeque::<(String, isize)>::new();
        let mut visited = HashSet::<String>::new();

        queue.push_back((from.to_string(), 0));

        loop {
            if queue.is_empty() { break; }

            let (key, step) = queue.pop_front().unwrap();

            if visited.contains(&key) { continue; }

            if key == to { return Some(step); }

            visited.insert(key.to_string());

            valves.get(&key).unwrap()
                .tunnels
                .iter()
                .filter(|v| ! visited.contains(&v.to_string()))
                .for_each(|v| {
                    queue.push_back((v.to_string(), step + 1));
                });
        }

        None
    }

    fn get_distance(&self, from: &str, to: &str) -> Option<isize> {
        let (fs, ts) = (from.to_string(), to.to_string());

        let key = (fs.to_string(), ts.to_string());

        self.distances.get(&key).or_else(|| {
            let key = (ts, fs);
            self.distances.get(&key)
        }).copied()
    }

    // If this path is found, returns the pressure release on the way there
    // and the total pressure release should we not go elsewhere after,
    // and the distance we can travel
    fn get_travel_value(
        &self,
        minutes_left_before: isize,
        pressure_per_minute: isize,
        from: &str,
        to: &str,
        target_flow_rate: isize
    ) -> Option<(isize, isize, isize)> {
        let dist = self.get_distance(from, to)? + 1;

        if dist > minutes_left_before {
            return Some((minutes_left_before * pressure_per_minute, 0, minutes_left_before))
        }

        Some((
            dist * pressure_per_minute,
            (minutes_left_before - dist) * (pressure_per_minute + target_flow_rate),
            dist,
        ))
    }
}

#[derive(Debug, PartialEq)]
struct Valve {
    id: String,
    flow_rate: isize,
    tunnels: Vec<String>,
}

impl Valve {
    fn parse(input: &str) -> Self {
        let split: Vec<&str> = input.split(" ").collect();

        let id = split[1].to_string();
        let flow_rate = 
            isize::from_str_radix(
                split[4].split("=").nth(1).unwrap().split(";").nth(0).unwrap(),
                10
            ).unwrap();

        let tunnels = split
            .iter()
            .skip(9)
            .map(|v| v.split(",").nth(0).unwrap().to_string())
            .collect::<Vec<String>>();

        Valve { id, flow_rate, tunnels }
    }
}

#[derive(Debug, PartialEq)]
struct Volcano {
    valves: HashMap<String, Valve>,
    distance_map: DistanceMap,
}

impl Volcano {
    fn parse(input: &Vec<String>) -> Volcano {
        let valves = input
            .iter()
            .map(|l| Valve::parse(l))
            .map(|v| (v.id.to_string(), v))
            .collect::<HashMap<String, Valve>>();

        let distance_map = DistanceMap::build(&valves);

        Volcano {
            valves,
            distance_map,
        }
    }

    // Whether it is useful to go for a path or not
    // If there is no time for the valve at that distance to release pressure
    // then it's a pointless path to travel to.
    fn is_useful(time_left: isize, distance: isize) -> bool {
        time_left > distance + 1
    }

    fn find_path(&self) -> isize {

        let origin = "AA";

        let mut first_valves = self.valves
            .iter()
            .filter(|(_, v)| v.flow_rate > 0)
            .map(|(id, v)| (id, v.flow_rate))
            .map(|(id, fr)| {
                let dist = DistanceMap::find_shortest_path_distance(&self.valves, origin, id).unwrap();
                let value = (30 - (dist + 1)) * fr;

                (id.to_string(), value, dist)
            })
            .collect::<Vec<(String, isize, isize)>>();

        first_valves.sort_by(|(_, v1, _), (_, v2, _)| v2.cmp(v1));

        let paths = first_valves
            .iter()
            .filter_map(|(vk, _, dist)| {
                let mins_left = 30 - (dist + 1);
                let fr = self.valves.get(&vk.to_string()).unwrap().flow_rate;
                self.continue_path(0, mins_left, fr, vec![vk.to_string()])
            })
            .map(|(t_release, t_left, ppm, _)| { 
                t_release + t_left * ppm
            })
            .collect::<Vec<isize>>();

        // Best
        paths.iter().fold(0, |acc, val| { 
            if *val > acc { *val } else { acc }
        })
    }

//    fn find_path_with_elephant(&self) -> isize {
//        let origin = "AA";
//
//        let mut first_valves = self.valves
//            .iter()
//            .filter(|(_, v)| v.flow_rate > 0)
//            .map(|(id, v)| (id, v.flow_rate))
//            .map(|(id, fr)| {
//                let dist = DistanceMap::find_shortest_path_distance(&self.valves, origin, id).unwrap();
//                let value = (26 - (dist + 1)) * fr;
//
//                (id.to_string(), value, dist)
//            })
//            .collect::<Vec<(String, isize, isize)>>();
//
//        first_valves.sort_by(|(_, v1, _), (_, v2, _)| v2.cmp(v1));
//
//        let mut start_combos: Vec<((String, isize, isize), (String, isize, isize))> = vec![];
//
//        loop {
//            let v1 = first_valves.pop().unwrap();
//
//            for v2 in first_valves.iter() {
//                start_combos.push(v1, v2); // but properly
//            }
//
//            if first_valves.len() == 1 { break; }
//        }
//
//        let paths = start_combos
//            .iter()
//            .filter_map(|s1, s2| {
//                let (vk1, _, dist1) = s1;
//                let (vk2, _, dist2) = s2;
//
//                let mins_left1 = 26 - (dist1 + 1);
//                let minst_left2 = 26 - (dist2 + 1);
//                let fr1 = self.valves.get(&vk1.to_string()).unwrap().flow_rate;
//                let fr2 = self.valves.get(&vk2.to_string()).unwrap().flow_rate;
//                let fr = fr1 + fr2;
//                self.continue_path_with_elephant(
//                    0, mins_left1, mins_left2, fr, vec![vk1.to_string()], vec![vk2.to_string()],
//                )
//            })
//            .map(|(t_release, t_left1, t_left2, ppm, _, _)| {
//                t_release + min(t_left1, t_left2) * ppm
//            })
//            .collect::<Vec<isize>>();
//
//
//        // Best
//        paths.iter().fold(0, |acc, val| { 
//            if *val > acc { *val } else { acc }
//        })
//    }

    fn continue_path(
        &self,
        total_release: isize,
        mins_left: isize,
        pressure_per_minute: isize,
        visited: Vec<String>
    ) -> Option<(isize, isize, isize, Vec<String>)> {
        let current = visited.iter().last().unwrap().to_string();

        let mut remaining = self.valves.keys().filter(|k| {
            let v = self.valves.get(&k.to_string()).unwrap();
            v.flow_rate > 0 && ! visited.iter().any(|s| &s == k)
        })
        .filter(|k| {
            let dist = self.distance_map.get_distance(&current, k).unwrap();
            Self::is_useful(mins_left, dist)
        })
        .filter_map(|k| {
            let (n_released, _, dist) = self.distance_map.get_travel_value(
                mins_left,
                pressure_per_minute,
                &current,
                k,
                self.valves.get(k).unwrap().flow_rate,
            ).unwrap();

            let m_left = mins_left - dist;
            let t_rel = total_release + n_released;
            let p_per_min = pressure_per_minute + self.valves.get(k).unwrap().flow_rate;
            let mut vis = visited.iter().map(|s| s.to_string()).collect::<Vec<String>>();
            vis.push(k.to_string());

            self.continue_path(t_rel, m_left, p_per_min, vis)
        })
        .collect::<Vec<(isize, isize, isize, Vec<String>)>>();

        if remaining.len() == 0 {
            return Some((total_release, mins_left, pressure_per_minute, visited));
        }

        remaining.sort_by(|a, b| {
            let (b_released, b_t_left, b_fr, _) = b;
            let (a_released, a_t_left, a_fr, _) = a;

            (b_released + b_t_left * b_fr).cmp(&(a_released + a_t_left * a_fr))
        });

        remaining.first().cloned()
    }

//    fn continue_path_with_elephant(
//        &self,
//        total_release: isize,
//        mins_left1: isize,
//        mins_left2: isize,
//        pressure_per_minute: isize,
//        visited1: Vec<String>,
//        visited2: Vec<String>,
//    ) -> Option<(isize, isize, isize, Vec<String>, Vec<String>)> {
//        let current1 = visited1.iter().last().unwrap().to_string();
//        let current2 = visited2.iter().last().unwrap().to_string();
//
//
//        // Well this is where I got to. 
//        // This bit below, copied mostly from continue_path needs
//        // further modification to process two actors moving through the volcano.
//        let mut remaining = self.valves.keys().filter(|k| {
//            let v = self.valves.get(&k.to_string()).unwrap();
//            v.flow_rate > 0 && ! visited1.iter().any(|s| &s == k) && ! visited2.iter().any(|s| &s == k)
//        })
//        .filter(|k| {
//            let dist = self.distance_map.get_distance(&current, k).unwrap();
//            Self::is_useful(mins_left, dist)
//        })
//        .filter_map(|k| {
//            let (n_released, _, dist) = self.distance_map.get_travel_value(
//                mins_left,
//                pressure_per_minute,
//                &current,
//                k,
//                self.valves.get(k).unwrap().flow_rate,
//            ).unwrap();
//
//            let m_left = mins_left - dist;
//            let t_rel = total_release + n_released;
//            let p_per_min = pressure_per_minute + self.valves.get(k).unwrap().flow_rate;
//            let mut vis = visited.iter().map(|s| s.to_string()).collect::<Vec<String>>();
//            vis.push(k.to_string());
//
//            self.continue_path(t_rel, m_left, p_per_min, vis)
//        })
//        .collect::<Vec<(isize, isize, isize, Vec<String>)>>();
//
//        if remaining.len() == 0 {
//            return Some((total_release, mins_left, pressure_per_minute, visited));
//        }
//
//        remaining.sort_by(|a, b| {
//            let (b_released, b_t_left, b_fr, _) = b;
//            let (a_released, a_t_left, a_fr, _) = a;
//
//            (b_released + b_t_left * b_fr).cmp(&(a_released + a_t_left * a_fr))
//        });
//
//        remaining.first().cloned()
//    }
}



#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_valve_parse() {
        assert_eq!(
            Valve::parse("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"),
            Valve { 
                id: "AA".to_string(),
                flow_rate: 0,
                tunnels: vec![ "DD".to_string(), "II".to_string(), "BB".to_string() ],
            },
        );

        assert_eq!(
            Valve::parse("Valve JJ has flow rate=21; tunnel leads to valve II"),
            Valve {
                id: "JJ".to_string(),
                flow_rate: 21,
                tunnels: vec![ "II".to_string() ],
            },
        );
    }

    /*
    #[test]
    fn test_volcano_parse() {
        assert_eq!(
            Volcano::parse(
                &vec![
                    "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_string(),
                    "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_string(),
                    "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_string(),
                ],
            ),
            Volcano {
                valves: HashMap::from([
                    ("AA".to_string(), Valve { 
                        id: "AA".to_string(),
                        flow_rate: 0,
                        tunnels: vec!["DD".to_string(), "II".to_string(), "BB".to_string()],
                    }),
                    ("BB".to_string(), Valve { 
                        id: "BB".to_string(),
                        flow_rate: 13,
                        tunnels: vec!["CC".to_string(), "AA".to_string()],
                    }),
                    ("CC".to_string(), Valve { 
                        id: "CC".to_string(),
                        flow_rate: 2,
                        tunnels: vec!["DD".to_string(), "BB".to_string()],
                    }),
                ]),
                open_valves: HashSet::new(),
            },
        )
    }*/

    #[test]
    fn test_distance_map() {
        let mut volcano = Volcano::parse(
            &vec![
                "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_string(),
                "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_string(),
                "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_string(),
                "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE".to_string(),
                "Valve EE has flow rate=3; tunnels lead to valves FF, DD".to_string(),
                "Valve FF has flow rate=0; tunnels lead to valves EE, GG".to_string(),
                "Valve GG has flow rate=0; tunnels lead to valves FF, HH".to_string(),
                "Valve HH has flow rate=22; tunnel leads to valve GG".to_string(),
                "Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_string(),
                "Valve JJ has flow rate=21; tunnel leads to valve II".to_string(),
            ],
        );

        let distance_map = DistanceMap::build(&volcano.valves);

        assert_eq!(distance_map.get_distance("BB", "DD").unwrap(), 2);
        assert_eq!(distance_map.get_distance("JJ", "HH").unwrap(), 7);
    }

    #[test]
    fn test_pressure_release_total() {
        let mut volcano = Volcano::parse(
            &vec![
                "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB".to_string(),
                "Valve BB has flow rate=13; tunnels lead to valves CC, AA".to_string(),
                "Valve CC has flow rate=2; tunnels lead to valves DD, BB".to_string(),
                "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE".to_string(),
                "Valve EE has flow rate=3; tunnels lead to valves FF, DD".to_string(),
                "Valve FF has flow rate=0; tunnels lead to valves EE, GG".to_string(),
                "Valve GG has flow rate=0; tunnels lead to valves FF, HH".to_string(),
                "Valve HH has flow rate=22; tunnel leads to valve GG".to_string(),
                "Valve II has flow rate=0; tunnels lead to valves AA, JJ".to_string(),
                "Valve JJ has flow rate=21; tunnel leads to valve II".to_string(),
            ],
        );

        assert_eq!(
            volcano.find_path(),
            1651,
        );



    }

}



   
