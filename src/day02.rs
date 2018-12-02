use edit_distance::edit_distance;

pub fn run() {
    let data = include_str!("../data/day02.txt");

    // Part 1
    // in all lines of data find the total count of lines that have at least 1 duplicate set of
    // letters and/or at least 1 set of triplicate letters.
    println!("The checksum is: {}", generate_checksum(data));

    // Part 2
    // Find the letters common between the two box ids that only differ by 1 character
    let ids = find_closest_ids(data);

    // find the matching characters in both ids
    let matching = ids
        .0
        .chars()
        .into_iter()
        .zip(ids.1.chars().into_iter())
        .fold(String::new(), |acc, c| {
            if c.0 == c.1 {
                return format!("{}{}", acc, c.0);
            }

            acc
        });

    println!("Matching characters: {}", matching);
}

// generate and return a checksum in lines of data by multiplying the occurances of lines with
// duplicate letters with the lines that have triplicate letters.
fn generate_checksum(data: &str) -> u32 {
    let counts: (u32, u32) = data.lines().fold((0, 0), |acc, line| {
        let mut chars: Vec<char> = line.chars().collect();
        chars.dedup();

        let mut double_letters = 0;
        let mut tripple_letters = 0;
        for c in chars {
            match line.matches(c).count() {
                2 => double_letters = 1,
                3 => tripple_letters = 1,
                _ => (),
            }
        }

        (acc.0 + double_letters, acc.1 + tripple_letters)
    });

    return counts.0 * counts.1;
}

// returns the first 2 lines to have an edit distance of 1
fn find_closest_ids(data: &str) -> (&str, &str) {
    for line1 in data.lines() {
        for line2 in data.lines() {
            if edit_distance(line1, line2) == 1 {
                return (line1, line2);
            }
        }
    }

    return ("", "");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_checksum() {
        let checksum = generate_checksum(include_str!("../data/day02_sample.txt"));
        assert_eq!(12, checksum);
    }

    #[test]
    fn test_find_closest_strings() {
        let lines = find_closest_ids(include_str!("../data/day02_sampleb.txt"));

        assert_eq!("fghij", lines.0);
        assert_eq!("fguij", lines.1);
    }
}
