/*
Proboscidea Volcanium
*/
use std::env;
use std::collections::{HashMap,HashSet,VecDeque};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Day16Error {
    ParseVavleError,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Valve {
    name: String,
    flow_rate: usize, // ppm
    connected_tunnels: Vec<String>,
    is_open: bool,
}

impl Valve {
    fn new(name:String, flow_rate:usize, connected_tunnels:Vec<String>, is_open:bool) -> Self {
        Self{name, flow_rate, connected_tunnels, is_open}
    }

    fn pressure(&self, minutes:usize) -> usize {
        self.flow_rate * minutes
    }
}

impl FromStr for Valve {
    type Err = Day16Error;

    fn from_str(v:&str) -> Result<Self, Self::Err> {
        let cleaned = v
            .replace("Valve ", "")
            .replace("has flow rate=", "")
            .replace(" tunnels lead to valves ", "") 
            .replace(" tunnels lead to valve ", "") // oh so tricky
            .replace(" tunnel leads to valves ", "") // oh so tricky
            .replace(" tunnel leads to valve ", ""); // oh so tricky
        match cleaned.split_once(";") {
            Some((name_and_rate, tunnels)) => {
                match (name_and_rate.split_once(" "), tunnels) {
                    (Some((name, rate)), tunnels) => {
                        match rate.parse::<usize>() {
                            Ok(flow_rate) => {
                                Ok(Self{
                                    name: name.to_string(),
                                    flow_rate,
                                    connected_tunnels: tunnels.replace(" ", "").split(",").map(|t| t.to_string()).collect::<Vec<String>>(),
                                    is_open: false,
                                })
                            },
                            _ => Err(Day16Error::ParseVavleError),
                        }
                    },
                    (None, _) => Err(Day16Error::ParseVavleError),
                }
            },
            None => Err(Day16Error::ParseVavleError),
        }
    }
}

impl std::fmt::Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.flow_rate)
    }
}

impl Default for Valve {
    fn default() -> Self {
        Self{name: "".to_string(), flow_rate: 0, connected_tunnels: vec![], is_open: false}
    }
}

const OPEN_VALVE_OPEN_COST:usize = 1;
const TRAVEL_COST:usize = 1;
const MAX_VALVES:usize = 56;

struct ValveFlows<const VALVE_COUNT:usize> {
    valve: [[usize; VALVE_COUNT]; VALVE_COUNT],
}

impl Default for ValveFlows<MAX_VALVES> {
    fn default() -> Self {
        ValveFlows{ valve: [ [0; MAX_VALVES]; MAX_VALVES]}
    }
}


fn main()
{
    let args:Vec<String> = env::args().collect();
    if let None = args.get(1) {
        println!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    if let Ok(data) = read_to_string(&args[1]) {
        // let tunnel_valves = TunnelValves::new(data.lines().filter_map(|l| l.parse::<Valve>().ok()).collect::<Vec<Valve>>());
        let valves = data.lines().filter_map(|l| l.parse::<Valve>().ok()).collect::<Vec<Valve>>();

        let valve_count = valves.len();
        let mut flows = ValveFlows::default();
        let valve_map:HashMap<String, usize> = valves.iter().enumerate().map(|(idx, v)| (v.name.clone(), idx)).collect::<HashMap<String, usize>>();

        // get the flows
        // for x in 0..1 { // valve_count {
            let x = 0;
            let xvalve = &valves[valve_map["AA"]];
            for (y, connected) in xvalve.connected_tunnels.iter().enumerate() {
                let yvalve = &valves[valve_map[connected]];

                // track minutes remaining and total flow
                let mut valves_to_visit:VecDeque<(&Valve, usize, usize)> = VecDeque::from([(yvalve, 28, 0)]);
                let mut valves_opened:HashSet<&Valve> = HashSet::new();
                while ! valves_to_visit.is_empty() {
                    if let Some((valve, mut minutes_remaining, mut flow)) = valves_to_visit.pop_front() {
                        // println!("== minute {} at {} ==", 30-minutes_remaining, valve);
                            if valve.flow_rate > 0 && minutes_remaining > 0 {
                                //if let None = valves_opened.get(valve) {
                                    minutes_remaining -= OPEN_VALVE_OPEN_COST;
                                    let lifetime_flow = valve.flow_rate * minutes_remaining;
                                    // println!("\tYou open valve {} (lifetime flow of {}).", valve, lifetime_flow);
                                    flow += lifetime_flow;
                                    //valves_opened.insert(valve);
                                //}
                            }
                        //}
                        if minutes_remaining == 0 {
                            println!("\t*** ran out of time with flow {}\n\n", flow);
                            flows.valve[x][y] = flow;
                            break;
                        }
                        for next_valve_name in &valve.connected_tunnels {
                            let next_valve_idx = valve_map[next_valve_name];
                            let next_valve = &valves[next_valve_idx];
                            //if next_valve.flow_rate > 0 {
                                valves_to_visit.push_back((next_valve, minutes_remaining - TRAVEL_COST, flow));
                            //}
                        }
                    }
                }
            }
        //}

        let mut max_flow = (0, 0, 0);
        for x in 0..valve_count {
            for y in 0..valve_count {
                if flows.valve[x][y] > max_flow.0 {
                    max_flow = (flows.valve[x][y], x, y);
                }
            }
        }
        dbg!(max_flow);
        println!("expected 2059");


        /*
        fn traverse<'a>(valve_map:&'a BTreeMap<String, Valve>, valve: &'a Valve, time_spent:usize, visited:&'a mut HashSet<&'a Valve>) -> usize {
            if time_spent >= 30 {
                return 0;
            }
            let flow = valve.flow_rate * (30 - time_spent);
            visited.insert(&valve);
            let visit_cost = if valve.flow_rate == 0 { CLOSED_VALVE_OPEN_COST } else { OPEN_VALVE_OPEN_COST };
            for tunnel in &valve.connected_tunnels {
                let inner_flow = traverse(valve_map, &valve_map[tunnel], time_spent + visit_cost + TRAVEL_COST, visited);
            }
            flow
        }
        let time = 0;
        let mut visited = HashSet::new();
        let cost = traverse(&valves_map, &valves[0], time, &mut visited);
        */

        /*
        dbg!(&valves_map);

        let mut cost_map:HashMap<(String, String), usize> = HashMap::new();
        for v in &valves {
            let first = &v.name;
            for v in &valves {
                if v.name.eq(first) {
                    continue
                }
                cost_map.insert((first.clone(), v.name.clone()), 1);
            }
        }
        // dbg!(cost_map);
        for ((start_name, end_name), cost) in &mut cost_map {
            let mut todo = VecDeque::from([(start_name.clone(), 0)]);
            while !todo.is_empty() {
                if let Some((valve, counter)) = todo.pop_front() {
                    if valve.eq(end_name) {
                        *cost = counter;
                        break;
                    }
                    let valve_name = valve.to_string();
                    println!("checking for {}", valve_name);
                    let tunnels = &valves_map[&valve_name].connected_tunnels;
                    dbg!(&tunnels);
                    for tunnel in tunnels {
                        if valve.eq(tunnel) {
                            continue;
                        }
                        todo.push_back((tunnel.clone(), counter + 1));
                    }
                    // for tunnel in valves_map[valve.to_string()]
                }
            }
        }

        // dbg!(&tunnel_valves);
        */
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_vale() {
        assert_eq!(
            "Valve AA has flow rate=20; tunnels lead to valves DD, II, BB".parse::<Valve>(),
            Ok(Valve::new("AA".to_string(), 20, vec!["DD".to_string(), "II".to_string(), "BB".to_string()], false))
        );
        assert_eq!(
            "Valve JJ has flow rate=21; tunnel leads to valve II".parse::<Valve>(),
            Ok(Valve::new("JJ".to_string(), 21, vec!["II".to_string()], false)),
        );
    }
}
