use std::path::Path;
use std::process::{Command, Stdio};
use std::error::Error;
use std::time::Instant;
use std::fs::{File, OpenOptions};
use csv::Writer;

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let output = Command::new("bombardier")
        .arg("-c")
        .arg("100")
        .arg("-n")
        .arg("50")
        .arg("http://localhost:9999/api/v1/users")
        .stdout(Stdio::piped())
        .output()?;
    
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut reqs_sec_avg: Option<f64> = None;
    let mut reqs_sec_stdev: Option<f64> = None;
    let mut reqs_sec_max: Option<f64> = None;
    let mut latency_avg_in_ms: Option<f64> = None;
    let mut latency_stdev_in_ms: Option<f64> = None;
    let mut latency_max_in_ms: Option<f64> = None;
    let mut http_code_1xx: Option<f64> = None;
    let mut http_code_2xx: Option<f64> = None;
    let mut http_code_3xx: Option<f64> = None;
    let mut http_code_4xx: Option<f64> = None;
    let mut http_code_5xx: Option<f64> = None;
    let mut http_other_codes: Option<f64> = None;
    let mut errors = 0;
    
    #[allow(non_snake_case)]
    let mut throughput_value_in_MB: Option<f64> = None;
    #[allow(non_snake_case)]
    #[allow(unused_assignments)]
    let mut column_names_exist = false;
    let file_exists = Path::new("output.csv").exists();
    if file_exists {
        let file = File::open("output.csv")?;
        let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(file);
         column_names_exist = reader.headers().is_ok();
    } else {
        column_names_exist = false;
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("output.csv")?;

    let mut writer = Writer::from_writer(file);
  if !column_names_exist {
    writer.write_record(&[
        "reqs_sec_avg",
        "reqs_sec_stdev",
        "reqs_sec_max",
        "latency_avg_in_ms",
        "latency_stdev_in_ms",
        "latency_max_in_ms",
        "http_code_1xx",
        "http_code_2xx",
        "http_code_3xx",
        "http_code_4xx",
        "http_code_5xx",
        "http_other_codes",
        "throughput_value_in_MB",
        "elapsed_time_secs",
        "errors"
    ])?;

    
}

    let stdout_lines_count = stdout.lines().count();


    for (line_number, line) in stdout.lines().enumerate() {
        let _ = line_number + 1;
        if line_number == 4 {
            reqs_sec_avg = line.split_whitespace().collect::<Vec<&str>>()[1].parse::<f64>().ok();
            reqs_sec_stdev = line.split_whitespace().collect::<Vec<&str>>()[2].parse::<f64>().ok();
            reqs_sec_max = line.split_whitespace().collect::<Vec<&str>>()[3].parse::<f64>().ok();
        }
        if line_number == 5 {
            let latency_avg_str = line.split_whitespace().collect::<Vec<&str>>()[1];
            let last_two_chars = &latency_avg_str[latency_avg_str.len() - 2..];
            let latency_avg_multiplier = match last_two_chars {
                "ms" => 1.0,
                "us" => 0.001,
                "ns" => 0.000001,
                "s" => 1000.0,
                _ => 1.0
            };

            let trimmed_return: f64 = match last_two_chars {
                "ms" => latency_avg_str.chars().take(latency_avg_str.len() - 2).collect::<String>().parse::<f64>().unwrap(),
                "us" => latency_avg_str.chars().take(latency_avg_str.len() - 2).collect::<String>().parse::<f64>().unwrap(),
                "ns" => latency_avg_str.chars().take(latency_avg_str.len() - 2).collect::<String>().parse::<f64>().unwrap(),
                "s" => latency_avg_str.chars().take(latency_avg_str.len() - 1).collect::<String>().parse::<f64>().unwrap(),
                _ => latency_avg_str.chars().take(latency_avg_str.len() - 1).collect::<String>().parse::<f64>().unwrap(),
                
            };
            let latency_avg_num = trimmed_return * latency_avg_multiplier;
            latency_avg_in_ms = Some(latency_avg_num);

            let latency_stdev_str = line.split_whitespace().collect::<Vec<&str>>()[2];
            let last_two_chars = &latency_stdev_str[latency_stdev_str.len() - 2..];
            let latency_stdev_multiplier = match last_two_chars {
                "ms" => 1.0,
                "us" => 0.001,
                "ns" => 0.000001,
                "s" => 1000.0,
                _ => 1000.0,
            };

            let trimmed_return: f64 = match last_two_chars {
                "ms" => latency_stdev_str.chars().take(latency_stdev_str.len() - 2).collect::<String>().parse::<f64>().unwrap(),
                "us" => latency_stdev_str.chars().take(latency_stdev_str.len() - 2).collect::<String>().parse::<f64>().unwrap(),
                "ns" => latency_stdev_str.chars().take(latency_stdev_str.len() - 2).collect::<String>().parse::<f64>().unwrap(),
                "s" => latency_stdev_str.chars().take(latency_stdev_str.len() - 1).collect::<String>().parse::<f64>().unwrap(),
                _ => latency_stdev_str.chars().take(latency_stdev_str.len() - 1).collect::<String>().parse::<f64>().unwrap(),
                
            };

            let latency_stdev_num = trimmed_return * latency_stdev_multiplier;
            latency_stdev_in_ms = Some(latency_stdev_num);
            let latency_max_str = line.split_whitespace().collect::<Vec<&str>>()[3];
            let last_two_chars = &latency_max_str[latency_max_str.len() - 2..];
            let latency_max_multiplier = match last_two_chars {
                "ms" => 1.0,
                "us" => 0.001,
                "ns" => 0.000001,
                "s" => 1000.0,
                _ => 1000.0,
            };

            let trimmed_return: f64 = match last_two_chars {
                "ms" => latency_max_str.chars().take(latency_max_str.len() - 2).collect::<String>().parse::<f64>().unwrap(),
                "us" => latency_max_str.chars().take(latency_max_str.len() - 2).collect::<String>().parse::<f64>().unwrap(),
                "ns" => latency_max_str.chars().take(latency_max_str.len() - 2).collect::<String>().parse::<f64>().unwrap(),
                "s" => latency_max_str.chars().take(latency_max_str.len() - 1).collect::<String>().parse::<f64>().unwrap(),
                _ => latency_max_str.chars().take(latency_max_str.len() - 1).collect::<String>().parse::<f64>().unwrap(),
                
            };

            let latency_max_num = trimmed_return * latency_max_multiplier;
            latency_max_in_ms = Some(latency_max_num);

        }
        if line_number == 7 {
            let split_items: Vec<&str> = line.split_whitespace().collect();
            for (index, item) in split_items.iter().enumerate() {
                if index == 2 {
                    let trimmed_item: String = item.chars().take(item.len() - 1).collect();
                    http_code_1xx = trimmed_item.parse::<f64>().ok();
                }
                if index == 5 {
                    let trimmed_item: String = item.chars().take(item.len() - 1).collect();
                    http_code_2xx = trimmed_item.parse::<f64>().ok();
                }

                if index == 8 {
                    let trimmed_item: String = item.chars().take(item.len() - 1).collect();
                    http_code_3xx = trimmed_item.parse::<f64>().ok();
                }

                if index == 11 {
                    let trimmed_item: String = item.chars().take(item.len() - 1).collect();
                    http_code_4xx = trimmed_item.parse::<f64>().ok();
                }

                if index == 14 {
                    http_code_5xx = item.parse::<f64>().ok();
                }
            }
        }
        if line_number == 8 {
            http_other_codes = Some(line.split_whitespace().collect::<Vec<&str>>()[2].parse::<f64>().ok().unwrap());
        }
        if line_number == 9 && line.contains("Throughput:") {
            let throughput = line.split_whitespace().collect::<Vec<&str>>()[1].to_string();
            let last_four_chars = &throughput[throughput.len() - 4..];
            let throughput_multiplier = match last_four_chars {
                "MB/s" => 1.0,
                "KB/s" => 0.001,
                "GB/s" => 1000.0,
                _ => 1.0
            };

            let trimmed_return: f64 = throughput.chars().take(throughput.len() - 4).collect::<String>().parse::<f64>().unwrap();
            throughput_value_in_MB = Some(trimmed_return * throughput_multiplier);
        }
        if line_number == 10 && stdout_lines_count > 9 {
        errors = line.split_whitespace().collect::<Vec<&str>>()[2].to_string().parse::<i16>().unwrap();
        }
        if line_number == 13 && stdout_lines_count > 12 {
            let throughput = line.split_whitespace().collect::<Vec<&str>>()[1].to_string();
            let last_four_chars = &throughput[throughput.len() - 4..];
            let throughput_multiplier = match last_four_chars {
                "MB/s" => 1.0,
                "KB/s" => 0.001,
                "GB/s" => 1000.0,
                _ => 1.0
            };

            let trimmed_return: f64 = throughput.chars().take(throughput.len() - 4).collect::<String>().parse::<f64>().unwrap();
            throughput_value_in_MB = Some(trimmed_return * throughput_multiplier);
        }
    }






    writer.serialize((
        reqs_sec_avg,
        reqs_sec_stdev,
        reqs_sec_max,
        latency_avg_in_ms,
        latency_stdev_in_ms,
        latency_max_in_ms,
        http_code_1xx,
        http_code_2xx,
        http_code_3xx,
        http_code_4xx,
        http_code_5xx,
        http_other_codes,
        throughput_value_in_MB,
        elapsed_time.as_nanos(),
        errors
    ))?;
    
    writer.flush()?;
    
    Ok(())
}
