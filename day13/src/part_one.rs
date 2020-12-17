// part_one.rs

pub fn earliest_bus_times_minutes(notes: &String) -> Option<usize> {
    let data: Vec<&str> = notes.split("\n").collect();

    let timestamp: usize = data[0].parse().unwrap();
    let list_bus_id: Vec<usize> = data[1]
        .split(",")
        .filter(|&x| x != "x")
        .flat_map(|x| x.parse())
        .collect();

    let minor_time: usize = list_bus_id
        .iter()
        .map(|&id| id - timestamp % id )
        .min()?;

    let id: usize = list_bus_id
        .iter()
        .filter(|&id| ((timestamp + minor_time) % id) == 0)
        .fold(0, |_, &id| id);

    Some(id * minor_time)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_earliest_bus_times_minutes() {
        let notes = "939\n7,13,x,x,59,x,31,19".to_owned();

        assert_eq!(Some(295), earliest_bus_times_minutes(&notes));
    }
}
