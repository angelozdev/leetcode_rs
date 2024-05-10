pub fn two_crystal_balls(floors: &[bool]) -> usize {
    if floors.is_empty() {
        return 0;
    }

    let jumps = (floors.len() as f32).sqrt() as usize;
    let mut index: usize = 0;

    while let Some(&floor) = floors.get(index) {
        if floor {
            break;
        }
        index += jumps;
    }

    index = if index > (floors.len() - 1) {
        index - jumps
    } else {
        index
    };

    for i in index..floors.len() {
        if floors[i] {
            return i;
        }
    }

    floors.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let floors = vec![];
        assert_eq!(
            two_crystal_balls(&floors),
            0,
            "Should return 0 for an empty array"
        );
    }

    #[test]
    fn test_all_false() {
        let floors = vec![false, false, false, false, false];
        assert_eq!(
            two_crystal_balls(&floors),
            5,
            "Should return the length of the array when no true is found"
        );
    }

    #[test]
    fn test_all_true() {
        let floors = vec![true, true, true, true, true];
        assert_eq!(
            two_crystal_balls(&floors),
            0,
            "Should return 0 as the first index with true"
        );
    }

    #[test]
    fn test_first_true() {
        let floors = vec![true];
        assert_eq!(
            two_crystal_balls(&floors),
            0,
            "Should return 0 as the first index with true"
        );
    }

    #[test]
    fn test_last_true() {
        let floors = vec![false, false, false, false, true];
        assert_eq!(
            two_crystal_balls(&floors),
            4,
            "Should return the last index as the first true"
        );
    }

    #[test]
    fn test_large_input() {
        let mut floors = vec![false; 10000];
        floors[9999] = true;
        println!("{:?}", floors);
        assert_eq!(
            two_crystal_balls(&floors),
            9999,
            "Should handle large input efficiently and find the last true"
        );
    }
}
