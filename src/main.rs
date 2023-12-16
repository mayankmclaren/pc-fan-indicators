use mqtt::publish;
use resources::Resources;
use std::{thread::sleep, time::Duration};

mod mqtt;
mod resources;
fn main() {
    let mut status = Resources::default();

    loop {
        let _ = status.get_cpu_usage();
        let _ = status.get_mem_resources();
        let _ = status.get_disk_usage();
        let available_ram_percent =
            status.memory.available_memory / status.memory.toatal_memory * 100.0;
        let available_disk_space = status.disk_space.free / status.disk_space.total * 100.0;
        let data = available_ram_percent.to_string()
            + ","
            + &available_disk_space.to_string()
            + ","
            + &status.cpu.current_utilization.to_string();

        print!("{}\n", data);
        let _ = publish(data);
        sleep(Duration::from_secs(10));
    }
}
