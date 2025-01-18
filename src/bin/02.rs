use advent_of_code::template::aoc_cli::check;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut reports: Vec<Vec<u32>> = vec![];
    for line in input.lines(){
        let items: Vec<u32> = line.split_whitespace()
            .filter_map(|item| item.parse::<u32>().ok())
            .collect();
        reports.push(items);
    }

    let mut safe_reports: u64 = 0;
    'outer: for report in reports {
        let mut ascending = false;
        let mut descending = false;
        for i in 1..report.len() {
            if report[i - 1] > report[i] {
                if ascending == true {
                    continue 'outer;
                }

                if report[i - 1] - report[i] > 3 {
                    continue 'outer;
                }

                descending = true;
            } else if report[i - 1] < report[i] {
                if descending == true {
                    continue 'outer;
                }

                if report[i] - report[i - 1] > 3 {
                    continue 'outer;
                }
                
                ascending = true;
            } else{ // neither descending or ascending (same number as before)
                continue 'outer;
            }
        }
        safe_reports += 1;
    }

    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut reports: Vec<Vec<u32>> = vec![];
    for line in input.lines(){
        let items: Vec<u32> = line.split_whitespace()
            .filter_map(|item| item.parse::<u32>().ok())
            .collect();
        reports.push(items);
    }

    let mut safe_reports: u64 = 0;
    for report in reports {
        if check_report(report.clone()) == false {
            for i in 0..report.len() {
                let mut rep = report.clone();
                rep.remove(i);
                if check_report(rep) == true {
                    safe_reports += 1;
                    break;
                };
            }
        } else {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
}

fn check_report(report: Vec<u32>) -> bool {
    let mut ascending = false;
    let mut descending = false;
    for i in 1..report.len() {
        if report[i - 1] > report[i] {
            if ascending == true {
                return false;
            }

            if report[i - 1] - report[i] > 3 {
                return false;
            }

            descending = true;
        } else if report[i - 1] < report[i] {
            if descending == true {
                return false;
            }

            if report[i] - report[i - 1] > 3 {
                return false;
            }
            
            ascending = true;
        } else{ // neither descending or ascending (same number as before)
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(314));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(373));
    }
}
