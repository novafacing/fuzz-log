pub mod report {
    use chrono::{DateTime, Utc};
    use schemars::JsonSchema;

    #[derive(JsonSchema, Debug, Clone)]
    /// Optional information about a CPU or group of CPUs on a host
    pub struct Cpu {
        /// The CPU architecture
        architecture: Option<String>,
        /// The CPU model name (e.g. Intel(R) Core(TM) i7-7700HQ CPU)
        model: Option<String>,
        /// The number of cores present of this CPU type
        cores: Option<u32>,
        /// The number of threads present of this CPU type If this is not present, it is
        /// assumed that each core has one thread
        threads: Option<u32>,
        /// The clock speed of this CPU in MHz
        frequency: Option<u32>,
    }

    #[derive(JsonSchema, Debug, Clone)]
    /// Optional information about memory on a host
    pub struct Memory {
        /// The total amount of memory in the specified unit
        /// (assume bytes if not specified)
        total: Option<u64>,
        /// The unit of memory (e.g. 'B', 'KB', 'MB', 'GB', 'TB', 'PB')
        unit: Option<String>,
    }

    #[derive(JsonSchema, Debug, Clone)]
    pub struct Host {
        /// The hostname of this host
        hostname: String,
        /// Optional list of CPUs on this host
        cpus: Option<Vec<Cpu>>,
        /// Optional list of memory on this host
        memory: Option<Vec<Memory>>,
        /// Optional OS string (e.g. from `uname -a`)
        os: Option<String>,
    }

    #[derive(JsonSchema, Debug, Clone)]
    // NOTE: Known common/semi-common fuzzers found with:
    // 'stars:>150 pushed:>2022-01-01 fuzzer'
    // - ffuf/ffuf (Go web fuzzer)
    // - google/syzkaller ( Kernel fuzzer)
    // - AFLplusplus/AFLplusplus (AFL++)
    // - google/honggfuzz
    // - crytic/echidna (Ethereum smart contract fuzzer)
    // - googleprojectzero/winafl (AFL for Windows)
    // - jtpereyda/boofuzz (Network protocol fuzzer)
    // - googleprojectzero/fuzzilli (Javascript engine fuzzer)
    // - AFLplusplus/LibAFL (AFL++ library)
    // - googleprojectzero/domato (DOM fuzzer)
    // - google/gofuzz (Go fuzzer)
    // - rust-fuzz/(afl.rs/cargo-fuzz) (Rust fuzzers/helpers)
    // - 0vercl0k/wtf (Bespoke fuzzer for windows, linux)
    // - Endava/cats (REST API fuzzer)
    // - googleprojectzero/jackalope (Fuzzer for windows, macos, linux, android, black box)
    // - CodeIntelligenceTesting/jazzer (Java fuzzer)
    // - AFLNet (AFL for network protocols)
    // - nccgroup/fuzzowski (Network protocol fuzzer)
    // - intel/kernel-fuzzer-for-xen-project (kernel fuzzer on xen)
    // - awslabs/snapchange (AWS KVM fuzzer)
    // - RESTLer (REST API fuzzer)
    // - CodeIntelligenceTesting/jazzer.js (Javascript fuzzer)
    // - Google/atheris (Python fuzzer)
    // - HBFA (Hardware based firmware analyzer)
    //
    // 'stars:>150 pushed:<2022-01-01 fuzzer'
    // - google/AFL (The classic, many are left out here which are forks of AFL/WinAFL/etc)
    // - Battelle/sandsifter (CPU instruction fuzzer)
    // - aoh/radamsa (Black-box, fully random input generator)
    // - nccgroup/TriforceAFL (AFL+QEMU Full System Emulation Fuzzer)
    // - (RUB-SysSec/IntelLabs)/kAFL (Kernel AFL for Linux+Windows on KVM)
    // - MozillaSecurity/peach (Grammar based fuzzer)
    pub enum Fuzzer {}

    /// # Standard fuzzer final report format
    /// This format can be implemented by fuzzing engines to provide a standard
    /// summary output format.
    #[derive(JsonSchema, Debug, Clone)]
    pub struct Report {
        /// The time this report was generated
        timestamp: DateTime<Utc>,
        /// Hosts available for running fuzzing campaigns
        hosts: Option<Vec<Host>>,
    }
}

pub mod log {
    pub struct Log {}

    pub struct Entry {}
}

#[cfg(test)]
pub mod test {
    use anyhow::Result;
    use schemars::schema_for;
    use serde_json::to_string_pretty;

    #[test]
    fn test_report() -> Result<()> {
        let schema = schema_for!(super::report::Report);
        println!("{}", to_string_pretty(&schema)?);
        Ok(())
    }
}
