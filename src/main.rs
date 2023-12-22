use std::collections::HashSet;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

const CGROUP_BASE_PATH: &str = "/sys/fs/cgroup";
const CGROUP_NAME: &str = "/user.slice";

fn check_for_new_pids(observed_pids: &mut HashSet<String>) {
    // Read the current list of PIDs from cgroup.procs
    let current_pids = fs::read_to_string(format!("{}{}/cgroup.procs", CGROUP_BASE_PATH, CGROUP_NAME))
        .expect("Error reading cgroup.procs");

    // Extract new PIDs
    let new_pids: Vec<String> = current_pids
        .split_whitespace()
        .filter(|pid| !observed_pids.contains(*pid))
        .map(|pid| pid.to_string())
        .collect();

    if !new_pids.is_empty() {
        new_pids.iter().for_each(|pid| println!("{}", pid));
        observed_pids.extend(new_pids);
    }
}

fn main() {
    let mut observed_pids = HashSet::new();
    loop {
        check_for_new_pids(&mut observed_pids);
        sleep(Duration::from_secs(5)); // Adjust the interval as needed
    }
}
