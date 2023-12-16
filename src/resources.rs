use anyhow::{Ok, Result};
use std::process::Command;

#[derive(Default, Debug)]
pub struct Resources {
    pub memory: Memory,
    pub cpu: CPU,
    pub disk_space: DiskSpace,
}
#[derive(Default, Debug)]
pub struct Memory {
    pub toatal_memory: f32,
    // pub used_memory: f32,
    pub available_memory: f32,
}
#[derive(Default, Debug)]
pub struct CPU {
    pub current_utilization: f32,
}
#[derive(Default, Debug)]
pub struct DiskSpace {
    pub total: f32,
    pub free: f32,
    // pub used: f32,
}

impl Resources {
    pub fn get_disk_usage(&mut self) -> Result<()> {
        let output = Command::new("df").output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        if let Some(sda) = output_str.lines().nth(2) {
            if let Some(total) = sda.split_whitespace().nth(1) {
                self.disk_space.total = total.parse::<f32>()? / 1048576.0;
            }
            if let Some(free) = sda.split_whitespace().nth(3) {
                self.disk_space.free = free.parse::<f32>()? / 1048576.0;
            }
            // if let Some(used) = sda.split_whitespace().nth(2) {
            //     self.disk_space.used = used.parse::<f32>()?/1048576.0;
            // }
        }
        Ok(())
    }

    pub fn get_cpu_usage(&mut self) -> Result<()> {
        let output = Command::new("top").arg("-bn1").output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        if let Some(third_line) = output_str.lines().nth(2) {
            if let Some(current_cpu_perct) = third_line.split_whitespace().nth(1) {
                self.cpu.current_utilization = current_cpu_perct.parse::<f32>()?;
            }
        }
        Ok(())
    }

    pub fn get_mem_resources(&mut self) -> Result<()> {
        let free = Command::new("free").output()?;
        let output_str = String::from_utf8_lossy(&free.stdout);

        if let Some(second_line) = output_str.lines().nth(1) {
            if let Some(available_ram) = second_line.split_whitespace().last() {
                self.memory.available_memory = available_ram.parse::<f32>()? / 1048576.0;
            }
            if let Some(toatl_ram) = second_line.split_whitespace().nth(1) {
                self.memory.toatal_memory = toatl_ram.parse::<f32>()? / 1048576.0;
            }
            //     if let Some(used_ram) = second_line.split_whitespace().nth(2) {
            //         self.memory.used_memory = used_ram.parse::<f32>()?/1048576.0;
            //     }
        }
        Ok(())
    }
}
