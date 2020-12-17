// part_two.rs

// Chinese remainder theorem (https://en.wikipedia.org/wiki/Chinese_remainder_theorem)
//
//  x ≡ ai (mod ni)
//
//  N = n1 * n2 * n3  * ... * nk
//  Nk = N / ni
//  Mi * Ni ≡ 1 (mod ni)
//
//  x = Sum(ai * Mi * Ni)
//

fn chinese(rem_mod: &[(usize, usize)]) -> Option<usize> {
    // rem_mod = (remainder, module) = (a, n)
    // calcule the N = n1 * n2 * n3 * ... * nk
    let N: usize = rem_mod
        .iter()
        .fold(1, |acc, (_, n)| acc * n );

    // Ni = N / ni
    let Ni_vec = rem_mod
        .iter()
        .map(|(_, n)| N/n);

    //  Mi * Ni ≡ 1 (mod ni)
    let Mi_vec = rem_mod
        .iter()
        .zip(Ni_vec.clone())
        .map(|((_, n), Ni)| {
            let mut Mi = 1;
            loop {
                if  (Mi * Ni) % n == 1 {
                    break Mi;
                } else {
                    Mi += 1;
                }
            }
        });

    let ai_Mi_Ni = rem_mod
        .iter()
        .zip(Mi_vec)
        .zip(Ni_vec)
        .map(|(((ai, _), Mi), Ni)| (ai, Mi, Ni));

    let x: usize = ai_Mi_Ni
        .fold(0, |acc, (ai, Mi, Ni)| acc + (ai * Mi * Ni) );

    Some(x % N)
}

// It doesn't work with the final notes because it takes a long time
pub fn earliest_timestamp_iterate(notes: &String, timestamp_start: usize) -> Option<usize> {
    let data: Vec<&str> = notes.split("\n").collect();

    let list_bus_id: Vec<(usize, usize)> = data[1]
        .split(",")
        .enumerate()
        .filter(|(_, x)| *x != "x")
        .map(|(i, x)| (i, x.parse().unwrap()))
        .collect();

    let mut found: bool = false;
    let mut t: usize = timestamp_start;
    let timestamp: usize = loop {
        for (i, id) in list_bus_id.iter() {
            match *i {
                0 if t % id != 0 => { found = false; break; },
                n if n > 0 && (n % id) != (id - (t % id)) => { found = false; break; },
                _ => (),
            }
        }
        if found {
            break t
        } else {
            found = true;
        }
        t += 1;
    };

    Some(timestamp)
}

pub fn earliest_timestamp_chinese(notes: &String) -> Option<usize> {
    let data: Vec<&str> = notes.split("\n").collect();

    let list_bus_id: Vec<(usize, usize)> = data[1]
        .split(",")
        .enumerate()
        .filter(|(_, x)| *x != "x")
        .map(|(i, x)| (i, x.parse().unwrap()))
        .collect();

    let res_mod: Vec<(usize, usize)> = list_bus_id
        .iter()
        .map(|(i, id)| {
            match *i {
                0 => (0, *id),
                n => (*id - n % *id, *id),
            }
        })
        .collect();

    let timestamp = chinese(&res_mod)?;

    Some(timestamp)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_earliest_timestamp() {
        let notes = "939\n7,13,x,x,59,x,31,19".to_owned();
        assert_eq!(Some(1068781), earliest_timestamp_iterate(&notes, 1));

        let notes = "0\n17,x,13,19".to_owned();
        assert_eq!(Some(3417), earliest_timestamp_iterate(&notes, 1));

        let notes = "0\n67,7,59,61".to_owned();
        assert_eq!(Some(754018), earliest_timestamp_iterate(&notes, 1));

        let notes = "0\n67,x,7,59,61".to_owned();
        assert_eq!(Some(779210), earliest_timestamp_iterate(&notes, 1));

        let notes = "0\n67,7,x,59,61".to_owned();
        assert_eq!(Some(1261476), earliest_timestamp_iterate(&notes, 1));

        // It test takes a long time
        // let notes = "0\n1789,37,47,1889".to_owned();
        // assert_eq!(Some(1202161486), earliest_timestamp_iterate(&notes, 1));

        let notes = "939\n7,13,x,x,59,x,31,19".to_owned();
        assert_eq!(Some(1068781), earliest_timestamp_chinese(&notes));

        let notes = "0\n17,x,13,19".to_owned();
        assert_eq!(Some(3417), earliest_timestamp_chinese(&notes));

        let notes = "0\n67,7,59,61".to_owned();
        assert_eq!(Some(754018), earliest_timestamp_chinese(&notes));

        let notes = "0\n67,x,7,59,61".to_owned();
        assert_eq!(Some(779210), earliest_timestamp_chinese(&notes));

        let notes = "0\n67,7,x,59,61".to_owned();
        assert_eq!(Some(1261476), earliest_timestamp_chinese(&notes));

        let notes = "0\n1789,37,47,1889".to_owned();
        assert_eq!(Some(1202161486), earliest_timestamp_chinese(&notes));
    }
}
