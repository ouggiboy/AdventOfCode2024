use std::fs;

const TESTING: bool = false;
const INPUT: &str = if TESTING {
    "sample.txt"
} else {
    "input.txt"
};

fn get_reports(path: &str) -> Vec<Vec<u8>> {
    let file = fs::read_to_string(path).expect("Unlucky, couldn't read filepath.");
    let mut reports: Vec<Vec<u8>> = Vec::new();

    for line in file.split('\n') {
        reports.push(
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u8>().expect("invalid number"))
                .collect()
        );
    }


    reports
}

fn is_safe(report: &Vec<u8>) -> bool {
    report.is_sorted_by(|&a, &b| a < b && a.abs_diff(b) <= 3)
    || report.is_sorted_by(|&a, &b| a > b && a.abs_diff(b) <= 3)
}

fn main() {
    let mut data = get_reports(INPUT);
    let mut p1_safe_reports = 0;
    
    for report in &data {
        if is_safe(report) { p1_safe_reports += 1; }
    }
    
    let mut p2_safe_reports = 0;

    for report in &mut data {
        let mut safe = is_safe(report);
        if !safe {
            for i in 0..report.len() {
                let temp = report.remove(i);
                if is_safe(report) { 
                    safe = true;
                    break;
                }
                else {
                    report.insert(i, temp);
                }
            }
        }
        if safe { 
            p2_safe_reports += 1; 
        }
    }
        

    println!("{} reports are safe", p1_safe_reports);
    println!("After dampening {} reports are safe", p2_safe_reports);
}