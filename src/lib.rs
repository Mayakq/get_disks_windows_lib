use std::collections::HashMap;

use windows_sys::Win32::Storage::FileSystem::GetLogicalDrives;

pub struct Disks(pub Vec<String>);

impl Default for Disks {
    fn default() -> Self {
        let mut result = Vec::with_capacity(26);
        unsafe {
            let bit_mask = GetLogicalDrives();
            for i in 0..26 {
                let bit_now = (bit_mask >> i) & 1;
                if bit_now == 1 {
                    result.push(((b'A' + i) as char).to_string() + ":/");
                }
            }
        }
        Self { 0: result }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_disks() {
        let result = Disks::default();
        assert!(result.0.len() > 0);
    }
}
