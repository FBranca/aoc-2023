use clap::Parser;
use num::Integer;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::rc::Rc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to parse
    #[arg(short, long)]
    input: String,
}

/*
 H H H H -> L
 x x x x -> H
        -- Low --> rx ?
*/
#[derive(Debug)]
enum EltType {
    FlipFlop,
    Conjuction(Vec<Rc<RefCell<Element>>>),
}

#[derive(Debug)]
struct Element {
    pub state: bool,
    pub targets: Vec<String>,
    pub predecessors: Vec<String>,
    elt_type: EltType,
}

impl Element {
    pub fn new_flipflop_rc(targets: Vec<String>) -> Rc<RefCell<Element>> {
        Rc::new(RefCell::new(Element {
            state: false,
            targets,
            predecessors: Vec::new(),
            elt_type: EltType::FlipFlop,
        }))
    }

    pub fn new_conjuction_rc(targets: Vec<String>) -> Rc<RefCell<Element>> {
        Rc::new(RefCell::new(Element {
            state: false,
            targets,
            predecessors: Vec::new(),
            elt_type: EltType::Conjuction(Vec::new()),
        }))
    }

    pub fn add_ref(&mut self, from: (&String, &Rc<RefCell<Element>>)) {
        self.predecessors.push(from.0.clone());
        if let EltType::Conjuction(refs) = &mut self.elt_type {
            refs.push(from.1.clone());
        }
    }

    pub fn trigger(&mut self, pulse: bool) -> bool {
        match &self.elt_type {
            EltType::FlipFlop => {
                if !pulse {
                    self.state = !self.state;
                    true
                } else {
                    false
                }
            }
            EltType::Conjuction(s) => {
                self.state = !s.iter().all(|elt| elt.as_ref().borrow().state);
                true
            }
        }
    }
}

type EltMap = HashMap<String, Rc<RefCell<Element>>>;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;

    let re = regex::Regex::new("([&%]*)(.+) -> (.+)").unwrap();
    let mut elts = EltMap::new();
    let mut broadcast = Vec::<String>::new();

    // build an internal representation of the modules
    BufReader::new(&file)
        .lines()
        .map_while(Result::ok)
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let cap = re.captures(&line).unwrap();

            let elt_type = cap.get(1).unwrap().as_str();
            let elt_name = cap.get(2).unwrap().as_str().to_string();
            let targets = cap
                .get(3)
                .unwrap()
                .as_str()
                .split(',')
                .map(str::trim)
                .map(String::from)
                .collect::<Vec<String>>();

            match elt_type {
                "%" => {
                    elts.insert(elt_name, Element::new_flipflop_rc(targets));
                }
                "&" => {
                    elts.insert(elt_name, Element::new_conjuction_rc(targets));
                }
                _ => {
                    assert_eq!(elt_name, "broadcaster");
                    broadcast = targets;
                }
            }
        });

    let mut rx_predecessor_id = Option::<String>::None;
    elts.iter().for_each(|(id, elt)| {
        elt.as_ref().borrow().targets.iter().for_each(|t| {
            if let Some(e) = elts.get(t) {
                e.borrow_mut().add_ref((&id, &elt));
            } else if t == "rx" {
                // assume the last stage is a Conjonction, just check it is
                assert!(matches!(
                    elt.as_ref().borrow().elt_type,
                    EltType::Conjuction(_)
                ));
                rx_predecessor_id = Some(id.clone());
            }
        });
    });

    let mut cycles = HashMap::<String, (u64, u64)>::new();
    if let Some(rx_predecessor_id) = rx_predecessor_id {
        println!("Rx predecessor is {:?}", rx_predecessor_id);
        let rx_predecessor = elts.get(&rx_predecessor_id).unwrap();
        rx_predecessor
            .as_ref()
            .borrow()
            .predecessors
            .iter()
            .for_each(|p| {
                cycles.insert(p.clone(), (0, 0));
            });
    }

    let mut count_low: u64 = 0;
    let mut count_high: u64 = 0;
    let mut cycle_detected = false;

    for i in 1..10000 {
        // push the button
        count_low += 1;
        let mut to_visit = VecDeque::<(bool, String)>::new();
        to_visit.extend(broadcast.iter().map(|id| (false, id.clone())));

        while let Some((pulse, id)) = to_visit.pop_front() {
            match pulse {
                true => count_high += 1,
                false => count_low += 1,
            }

            if let Some(elt) = elts.get_mut(&id) {
                if elt.borrow_mut().trigger(pulse) {
                    to_visit.extend(elt.as_ref().borrow().targets.iter().map(|s| {
                        let state = elt.as_ref().borrow().state;
                        if state {
                            if let Some(c) = cycles.get_mut(&id) {
                                *c = match c {
                                    (0, 0) => (1, i),
                                    (cnt, val) => {
                                        if (i % *val) == 0 {
                                            (*cnt + 1, *val)
                                        } else {
                                            println!(" /!\\ reset cycle {:?} {:?} {}", id, c, i);
                                            (1, i)
                                        }
                                    }
                                };
                                cycle_detected = cycles
                                    .iter()
                                    .fold(true, |acc, (_id, (cnt, _val))| acc & (*cnt > 1));
                            }
                        }
                        (state, s.clone())
                    }));
                }
            }
        }

        if i == 1000 {
            println!("Part 1 : {}", count_low * count_high);
        }
        if cycle_detected {
            let lcm = cycles
                .values()
                .map(|(_cnt, val)| val)
                .fold(1, |acc, c| acc.lcm(c));
            println!("Part 2 : {}", lcm);
            if i > 1000 {
                break;
            }
        }
    }

    Ok(())
}
