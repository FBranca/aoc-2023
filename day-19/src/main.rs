use clap::Parser;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to parse
    #[arg(short, long)]
    input: String,
}

#[derive(Debug)]
enum NodeValue {
    Leaf(String),
    Node(Box<Node_>),
}

#[derive(Debug)]
struct Node_ {
    pub attr: String,
    pub value: u32,
    pub less: NodeValue,
    pub eqmore: NodeValue,
}

type Workflows = HashMap<String, NodeValue>;

// dt{s<2042:bvt,a>2530:zd,sgj}
/*
       s/2042
    bvt      a/2531
           sgj
*/

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = File::open(args.input)?;
    let mut reader = BufReader::new(file);

    let re_line = regex::Regex::new("^([a-z]+)\\{(.+)\\}$").unwrap();
    let re = regex::Regex::new("(?:([a-z]+)(<|>)(A|R|[0-9]+):(A|R|[a-z]+))|(A|R|[a-z]+)").unwrap();
    let re_part = regex::Regex::new("([xmas])=([0-9]+)").unwrap();

    let mut workflows = Workflows::new();

    // Find map size
    (&mut reader)
        .lines()
        .map_while(Result::ok)
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            println!("{}", line);

            let cap_line = re_line.captures(&line).unwrap();
            let key = cap_line.get(1).unwrap().as_str();
            let expr = cap_line.get(2).unwrap().as_str();

            let mut n = Option::<NodeValue>::None;
            for cap in re
                .captures_iter(expr)
                .collect::<Vec<regex::Captures>>()
                .into_iter()
                .rev()
            {
                if let Some(value) = cap.get(5) {
                    // leaf
                    n = Some(NodeValue::Leaf(value.as_str().to_string()));
                } else {
                    // node
                    let attr = cap.get(1).unwrap().as_str();
                    let op = cap.get(2).unwrap().as_str();
                    let value = cap.get(3).unwrap().as_str().parse::<u32>().unwrap();
                    let then = cap.get(4).unwrap().as_str();

                    n = Some(match op {
                        "<" => NodeValue::Node(Box::new(Node_ {
                            attr: attr.to_string(),
                            value,
                            less: NodeValue::Leaf(then.to_string()),
                            eqmore: n.unwrap(),
                        })),
                        ">" => NodeValue::Node(Box::new(Node_ {
                            attr: attr.to_string(),
                            value: value + 1,
                            less: n.unwrap(),
                            eqmore: NodeValue::Leaf(then.to_string()),
                        })),
                        _ => panic!("invalid operator"),
                    });
                }
            }
            workflows.insert(key.to_string(), n.unwrap());
        });

    let sum = (&mut reader)
        .lines()
        .map_while(Result::ok)
        .take_while(|line| !line.is_empty())
        .fold(0, |acc, line| {
            let mut part_attr = HashMap::<String, u32>::new();
            for cap in re_part.captures_iter(&line) {
                let k = cap.get(1).unwrap().as_str().to_string();
                let v = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
                part_attr.insert(k, v);
            }

            let mut flow = "in";
            while !["R", "A"].contains(&flow) {
                let mut nvalue = workflows.get(flow).expect("workflow not found");
                while let NodeValue::Node(n) = nvalue {
                    let v = part_attr.get(&n.attr).expect("missing part attribute");
                    let child = match *v {
                        v if v < n.value => &n.less,
                        _ => &n.eqmore,
                    };
                    nvalue = child;
                }
                println!("{} {:?}", line, nvalue);
                flow = match nvalue {
                    NodeValue::Leaf(val) => val,
                    _ => unreachable!(),
                }
            }

            acc + match flow {
                "A" => part_attr.values().sum(),
                _ => 0,
            }
        });

    println!("Part 1: {}", sum);

    type Parts = HashMap<&'static str, std::ops::Range<u32>>;
    let part_attr = HashMap::<&str, std::ops::Range<u32>>::from([
        ("x", 1..4001),
        ("m", 1..4001),
        ("a", 1..4001),
        ("s", 1..4001),
    ]);
    let mut sum = 0;
    let mut to_visit = VecDeque::<(&NodeValue, Parts)>::new();
    to_visit.push_back((workflows.get("in").unwrap(), part_attr));
    while let Some((node, attrs)) = to_visit.pop_front() {
        match node {
            NodeValue::Leaf(leaf) => match leaf.as_str() {
                "R" => continue,
                "A" => {
                    sum += attrs.values().fold(1, |acc, v| {
                        println!("{:?}", v);
                        acc * v.len()
                    });
                    println!("{}", sum);
                    continue;
                }
                _ => {
                    to_visit.push_back((workflows.get(leaf).unwrap(), attrs));
                }
            },
            NodeValue::Node(node) => {
                let v = attrs
                    .get(&node.attr.as_str())
                    .expect("missing part attribute");
                if v.start < node.value {
                    let mut attrs = attrs.clone();
                    let v = attrs
                        .get_mut(&node.attr.as_str())
                        .expect("missing part attribute");
                    v.end = v.end.min(node.value);
                    to_visit.push_back((&node.less, attrs));
                }
                if v.end > node.value {
                    let mut attrs = attrs.clone();
                    let v = attrs
                        .get_mut(&node.attr.as_str())
                        .expect("missing part attribute");
                    v.start = v.start.max(node.value);
                    to_visit.push_back((&node.eqmore, attrs));
                }
            }
        }
    }

    println!("Part 2: {}", sum);

    Ok(())
}
