use std::path::Path;
use std::process::{Command, Stdio};
use std::error::Error;
use std::fs::{File, OpenOptions};
use csv::Writer;


fn main() -> Result<(), Box<dyn Error>> {
    for i in 1..4 {
        println!("Iteration: {}", i);


    // let filename = format!("spring_noredis_test_{}", i);
    // let arg = format!("../tests/spring_noredis_test/{}.js", filename);


    // let filename = format!("rust_test_noredis_{}", i);
    // let arg = format!("../tests/rust_noredis_test/{}.js", filename);


    // let filename = format!("rust_grpc_noredis_test_{}", i);
    // let arg = format!("../tests/rust_grpc_noredis_test/{}.js", filename);

    let filename = format!("rust_test_{}", i);
    let arg = format!("../tests/rust_redis_test/{}.js", filename);
    // let filename = format!("spring_redis_test_{}", i);
    // let arg = format!("../tests/spring_redis_test/{}.js", filename);
    // let filename = format!("rust_grpc_test_{}", i);
    // let arg = format!("../tests/rust_grpc_redis_test/{}.js", filename);




    let output = Command::new("k6")
        .arg("run")
        .arg(arg)
        .stdout(Stdio::piped())
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines: Vec<String> = Vec::new();

    for line in stdout.lines() {
        if is_interesting_line(line) {
            lines.push(line.to_string()); 
        }
    }

    drop(stdout);
    drop(output);


    let mut filtered_lines: Vec<String> = Vec::new();
    // Filter relevant lines
    for line in &lines {
        if line.contains("req_duration") || line.contains("iterations") || line.contains("iteration_duration") || line.contains("vus_max") {
            filtered_lines.push(line.to_string());
        }
    }
    println!("{:?}", filtered_lines);
    let iterations_total = filtered_lines[2].split_whitespace().collect::<Vec<&str>>()[1];
    let iterations_time_spen = filtered_lines[2].split_whitespace().collect::<Vec<&str>>()[2].chars().collect::<String>();
    let vus_max = filtered_lines[3].split_whitespace().collect::<Vec<&str>>()[1];


    let mut column_names_exist = false;
    let file_exists = Path::new("output.csv").exists();
    if file_exists {
        let file = File::open("output.csv")?;
        let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(file);
         column_names_exist = reader.headers().is_ok();
    } else {
        column_names_exist = false;
    }

    let filename = format!("{}.csv", filename);

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(filename)?;

    let mut writer = Writer::from_writer(file);
    if !column_names_exist {
    writer.write_record(&["req_duration_avg/ms", "req_duration_min/ms", "req_duration_med/ms", "req_duration_max/ms", "req_duration_p90/ms", "req_duration_p95/ms", "iteration_duration_avg/ms", "iteration_duration_min/ms", "iteration_duration_med/ms", "iteration_duration_max/ms", "iteration_duration_p90/ms", "iteration_duration_p95/ms", "iterations_total", "iterations_time_spen/ms", "vus_max"])?;
    }
    writer.write_record(&[
        parse_value(&parse_init_value(filtered_lines[0].clone(), 1, 4)), 
        parse_value(&parse_init_value(filtered_lines[0].clone(), 2, 4)), 
        parse_value(&parse_init_value(filtered_lines[0].clone(), 3, 4)),
        parse_value(&parse_init_value(filtered_lines[0].clone(), 4, 4)),
        parse_value(&parse_init_value(filtered_lines[0].clone(), 5, 6)),
        parse_value(&parse_init_value(filtered_lines[0].clone(), 6, 6)),
        parse_value(&parse_init_value(filtered_lines[1].clone(), 1, 4)), 
        parse_value(&parse_init_value(filtered_lines[1].clone(), 2, 4)), 
        parse_value(&parse_init_value(filtered_lines[1].clone(), 3, 4)),
        parse_value(&parse_init_value(filtered_lines[1].clone(), 4, 4)),
        parse_value(&parse_init_value(filtered_lines[1].clone(), 5, 6)),
        parse_value(&parse_init_value(filtered_lines[1].clone(), 6, 6)),

        iterations_total.to_string(), 
        parse_duration(&iterations_time_spen).to_string(), 
        vus_max.to_string()])?;

    drop(filtered_lines);
    writer.flush()?;
    drop(writer);
    }
    Ok(())
}


fn is_interesting_line(line: &str) -> bool {
    !line.contains("running")
    && !line.contains("default")
    && !line.contains("scenarios")
    && !line.contains("output: -")
    && !line.contains("script")
    && !line.contains("execution")
    && !line.contains("__________")
    && !line.trim().is_empty()
    && !line.contains("|")
}


fn parse_value(value: &str) -> String {
    println!("Value: {}", value);

    let value_float: f64 = if value.ends_with("ms") {
        value.trim_end_matches("ms").parse::<f64>().unwrap()
    } else if value.ends_with('s') && value.chars().rev().nth(1).unwrap().is_numeric() && value.chars().rev().nth(2).unwrap().is_numeric() {
        value.trim_end_matches('s').parse::<f64>().unwrap() * 1000.0
    } else if value.ends_with('s') && value.chars().rev().nth(1).unwrap().is_numeric() && !value.chars().rev().nth(2).unwrap().is_numeric() {
        let minutes = value.chars().rev().nth(3).unwrap().to_string();
        let seconds = value.chars().rev().nth(1).unwrap().to_string();
        let value_float = minutes.parse::<f64>().unwrap() * 60.0 + seconds.parse::<f64>().unwrap();
        value_float * 1000.0
    } 
    else if value.ends_with("ns") {
        value.trim_end_matches("ns").parse::<f64>().unwrap() / 1000000.0
    } else if value.ends_with("µs") {
        value.trim_end_matches("µs").parse::<f64>().unwrap() / 1000.0
    } else if value.ends_with('m') {
        value.trim_end_matches('m').parse::<f64>().unwrap() * 60.0 * 1000.0
    } else {
        value.parse::<f64>().unwrap()
    };

    value_float.to_string()
}

 fn parse_duration(duration: &str) -> f64 {
    let mut value = duration.trim_end_matches(|c| !char::is_numeric(c)).parse::<f64>().unwrap();
    match duration.chars().last().unwrap() {
        's' => value *= 1000.0,
        'n' => value /= 1000000.0,
        'm' => {}
        _ => unreachable!(),
    }
    value
}

fn parse_init_value(filtered_lines: String, element: usize, skip: usize) -> String {
    let value = filtered_lines.split_whitespace().collect::<Vec<&str>>()[element].chars().skip(skip).collect::<String>();
    value
}

