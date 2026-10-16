use rand::Rng;
struct Reading {
    day: String,
    high: i32,
}

struct Summary {
    average: f64,
    hottest_day: String,
    days_above: usize,
}
const DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
fn average_temp(log: &Vec<Reading>) -> f64 {
if log.len() == 0 {
return 0.0;
}
let mut total: f64 = 0.0;
for entry in log {
total += entry.high as f64;
}
return total / log.len() as f64;
}
fn hottest_day(log: &Vec<Reading>) -> usize {
    if log.len() == 0 {
        return 0;
    }

    let mut highest = log[0].high;
    let mut index: usize = 0;

    for i in 0..log.len() {
        if log[i].high > highest {
            highest = log[i].high;
            index = i;
        }
    }

    return index;
}
fn count_above(log: &Vec<Reading>, threshold: i32) -> usize {
    let mut count: usize = 0;

    for entry in log {
        if entry.high > threshold {
            count += 1;
        }
    }

    return count;
}
fn main() {
    let mut highs: Vec<Reading> = vec![
    Reading { day: "Mon".to_string(), high: 72 },
    Reading { day: "Tue".to_string(), high: 68 },
    Reading { day: "Wed".to_string(), high: 75 },
    Reading { day: "Thu".to_string(), high: 81 },
    Reading { day: "Fri".to_string(), high: 79 },
];
for i in 0..highs.len() {
    println!("{}: {}", DAYS[i], highs[i].high);
}

let mut rng = rand::thread_rng();

highs.push(Reading {
    day: "Sat".to_string(),
    high: rng.gen_range(60..=100),
});
highs.push(Reading {
    day: "Sun".to_string(),
    high: rng.gen_range(60..=100),
});

for i in 0..highs.len() {
println!("{}: {}", DAYS[i], highs[i].high);
}
println!("Average Temperature: {:.2}", average_temp(&highs));

let hottest = hottest_day(&highs);
println!("Hottest Day: {}", DAYS[hottest]);

println!("Days Above 75: {}", count_above(&highs, 75));
}


