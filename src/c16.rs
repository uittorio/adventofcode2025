use crate::utils;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct JunctionBox {
    x: i128,
    y: i128,
    z: i128,
}
pub fn run() -> i128 {
    let result = utils::challenge_file("15");
    let data = result
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split(",")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i128>().unwrap())
                .collect::<Vec<i128>>()
        })
        .map(|v| JunctionBox {
            x: v[0],
            y: v[1],
            z: v[2],
        })
        .collect::<Vec<JunctionBox>>();

    find(data)
}

#[derive(Debug)]
struct CircuitDistance {
    circuit_a: JunctionBox,
    circuit_b: JunctionBox,
    distance: i128,
}

#[derive(Debug)]
struct Circuit {
    junctions: Vec<JunctionBox>,
}
fn find(data: Vec<JunctionBox>) -> i128 {
    let mut distances: Vec<CircuitDistance> = vec![];
    for i1 in 0..data.len() {
        for i2 in (i1 + 1)..data.len() {
            let ele1 = &data[i1];
            let ele2 = &data[i2];

            let distance = ((ele2.x - ele1.x).pow(2))
                + ((ele2.y - ele1.y).pow(2))
                + ((ele2.z - ele1.z).pow(2));

            distances.push(CircuitDistance {
                circuit_a: *ele1,
                circuit_b: *ele2,
                distance,
            });
        }
    }

    distances.sort_by(|a, b| a.distance.cmp(&b.distance));

    let mut circuits: Vec<Circuit> = vec![];

    for j in data {
        circuits.push(Circuit { junctions: vec![j] });
    }

    let mut last_circuit: Option<CircuitDistance> = None;

    for distance in distances {
        let mut matched: (isize, isize) = (-1, -1);
        for circuit_index in 0..circuits.len() {
            let circuit_a = circuits[circuit_index].junctions.iter().any(|v| {
                v.x == distance.circuit_a.x
                    && v.y == distance.circuit_a.y
                    && v.z == distance.circuit_a.z
            });

            let circuit_b = circuits[circuit_index].junctions.iter().any(|v| {
                v.x == distance.circuit_b.x
                    && v.y == distance.circuit_b.y
                    && v.z == distance.circuit_b.z
            });

            match (circuit_a, circuit_b) {
                (false, false) => {}
                (false, true) => {
                    matched.1 = circuit_index as isize;
                }
                (true, false) => {
                    matched.0 = circuit_index as isize;
                }
                (true, true) => {
                    matched = (circuit_index as isize, circuit_index as isize);
                }
            }
        }

        match (matched.0, matched.1) {
            (-1, -1) => {}
            (-1, _) => {}
            (_, -1) => {}
            (index_a, index_b) => {
                if index_a != index_b {
                    let mut junctions = circuits[index_a as usize].junctions.clone();
                    circuits[index_b as usize].junctions.append(&mut junctions);
                    circuits.remove(index_a as usize);

                    if circuits.len() == 1 {
                        last_circuit = Some(distance);
                        break;
                    }
                }
            }
        }
    }

    match last_circuit {
        Some(lc) => return lc.circuit_a.x * lc.circuit_b.x,
        None => 0 as i128,
    }
}
