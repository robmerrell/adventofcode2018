use regex::Regex;
use std::collections::HashSet;

// Holds a claim
#[derive(Debug, PartialEq)]
struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Claim {
    fn squares_used(&self) -> HashSet<(i32, i32)> {
        let mut squares_used = HashSet::new();
        for x in self.x..self.x + self.width {
            for y in self.y..self.y + self.height {
                squares_used.insert((x, y));
            }
        }

        return squares_used;
    }
}

pub fn run() {
    let data = include_str!("../data/day03.txt");
    let mut claimed_locations = HashSet::new();

    // parse the claims
    let claims: Vec<Claim> = data.lines().map(|line| parse_claim(line)).collect();
    let claims_slice = claims.as_slice();

    // find all of the squares that overlap
    for claim1 in claims_slice {
        let mut overlap_count = 0;
        for claim2 in claims_slice {
            if claim1 != claim2 {
                for overlap in find_overlap(claim1, claim2) {
                    claimed_locations.insert(overlap);
                    overlap_count += 1;
                }
            }
        }

        // print out the claims that do not overlap
        if overlap_count == 0 {
            println!("Claim that doesn't overlap: {}", claim1.id);
        }
    }

    // find the total overlapped area
    println!(
        "total overlapped area: {}",
        claimed_locations.iter().count()
    );
}

// parse a line of input data into a Claim struct
fn parse_claim(claim: &str) -> Claim {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let capture = re.captures(claim).unwrap();

    let cap_as_i32 = |index| capture.get(index).unwrap().as_str().parse::<i32>().unwrap();

    return Claim {
        id: cap_as_i32(1),
        x: cap_as_i32(2),
        y: cap_as_i32(3),
        width: cap_as_i32(4),
        height: cap_as_i32(5),
    };
}

// find the overlap between two claims
fn find_overlap(claim1: &Claim, claim2: &Claim) -> Vec<(i32, i32)> {
    let mut overlap: Vec<(i32, i32)> = Vec::new();

    claim1
        .squares_used()
        .intersection(&claim2.squares_used())
        .into_iter()
        .for_each(|coords| overlap.push(*coords));

    return overlap;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_claim() {
        let claim = parse_claim("#1248 @ 560,201: 21x18");

        assert_eq!(claim.id, 1248);
        assert_eq!(claim.x, 560);
        assert_eq!(claim.y, 201);
        assert_eq!(claim.width, 21);
        assert_eq!(claim.height, 18);
    }

    #[test]
    fn test_squares_used() {
        let claim = parse_claim("#3 @ 5,5: 2x2");
        let squares_used = claim.squares_used();

        assert!(squares_used.contains(&(5, 5)));
        assert!(squares_used.contains(&(5, 6)));
        assert!(squares_used.contains(&(6, 5)));
        assert!(squares_used.contains(&(6, 6)));
    }

    #[test]
    fn test_find_overlap() {
        let claim1 = parse_claim("#1 @ 1,3: 4x4");
        let claim2 = parse_claim("#2 @ 3,1: 4x4");
        let claim3 = parse_claim("#3 @ 5,5: 2x2");

        let mut overlap = find_overlap(&claim1, &claim2);

        assert!(overlap.contains(&(3, 3)));
        assert!(overlap.contains(&(4, 3)));
        assert!(overlap.contains(&(3, 4)));
        assert!(overlap.contains(&(4, 4)));
        assert_eq!(4, overlap.iter().count());

        overlap = find_overlap(&claim1, &claim3);
        assert_eq!(0, overlap.iter().count());

        overlap = find_overlap(&claim2, &claim3);
        assert_eq!(0, overlap.iter().count());
    }
}
