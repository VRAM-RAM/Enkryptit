use sysinfo::System;

pub const MIB: u64 = 1024 * 1024;

/// Structure that contains & computes the parameters for folder encryption / decryption in multithreading.
pub struct FolderParallelizationParams {
    /// The maximum budget of memory for **one** entry.
    pub mem_per_entry: u64,
    /// The maximum number of entries that will be encrypted /decrypted in parallel 
    pub num_entries: usize,
}

impl FolderParallelizationParams {
    /// Computes the [`FolderParallelizationParams`] for a folder multithreading encryption / decryption.
    /// \
    /// It gets the `availabled_memory` and `cpus` (threads), and computes :
    /// - The maximum budget of memory for **one** entry
    /// - The maximum number of entries that will be encrypted / decrypted in parallel
    pub fn compute() -> Self {
        let mut system = System::new();
        system.refresh_memory();

        // We get the current available memory
        let available_memory = system.available_memory() / 4;

        // The available threads
        let cpus = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        // We compute a budget for Enkryptit! 
        // Here, it is the quarter of the amount of available memory
        let mem_budget = available_memory / 4;

        // The max memory per entry is the max memory an entry can use. (and so, in the parallelization code, we only parallelize entries that will only use an amount of memory < mem_per_entry)
        let mem_per_entry = (mem_budget / 4).max(MIB);

        // The number of entries that will be parallelized 
        let num_entries = if mem_per_entry == 0 {
            1
        } else {
            cpus.min((mem_budget / mem_per_entry) as usize).max(1)
        };

        Self { mem_per_entry, num_entries }
    }
} 

