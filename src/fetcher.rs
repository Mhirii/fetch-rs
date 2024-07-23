extern crate os_release;
use crate::utils::{memory_to_gb, ok_readout, seconds_to_string};
use libmacchina::{
    traits::{GeneralReadout as _, KernelReadout as _, MemoryReadout as _, PackageReadout as _},
    GeneralReadout, KernelReadout, MemoryReadout, PackageReadout,
};

pub struct FetchOpts {
    pub os: Option<bool>,
    pub kernel: Option<bool>,
    pub host: Option<bool>,
    pub user: Option<bool>,

    pub window_manager: Option<bool>,
    pub session: Option<bool>,
    pub terminal: Option<bool>,

    pub uptime: Option<bool>,
    pub cpu: Option<bool>,
    pub memory: Option<bool>,
    pub packages: Option<bool>,

    pub gpu1: Option<bool>,
    pub gpu2: Option<bool>,

    pub all: Option<bool>,
}

#[derive(Default)]
pub struct Fetched {
    pub os: Option<String>,
    pub kernel: Option<String>,
    pub host: Option<String>,
    pub user: Option<String>,

    pub window_manager: Option<String>,
    pub session: Option<String>,
    pub terminal: Option<String>,

    pub uptime: Option<String>,
    pub cpu: Option<String>,
    pub memory: Option<String>,
    pub packages: Option<String>,

    pub gpu1: Option<String>,
    pub gpu2: Option<String>,
}

pub struct Memory {
    total: String,
    // free: String,
    used: String,
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Memory {
    fn to_string(&self) -> String {
        format!("{}Gb / {}Gb", self.used, self.total)
    }
}

pub fn fetch_os(readout: &GeneralReadout) -> String {
    let read = readout.distribution();
    ok_readout(read).trim().to_string()
}

pub fn fetch_kernel(kernel_readout: &KernelReadout) -> String {
    let read = kernel_readout.os_release();
    ok_readout(read).trim().to_string()
}

pub fn fetch_host(readout: &GeneralReadout) -> String {
    let read = readout.hostname();
    ok_readout(read).trim().to_string()
}

pub fn fetch_window_manager(readout: &GeneralReadout) -> String {
    let read = readout.window_manager();
    ok_readout(read).trim().to_string()
}

pub fn fetch_session(readout: &GeneralReadout) -> String {
    let read = readout.session();
    ok_readout(read).trim().to_string()
}

pub fn fetch_uptime(readout: &GeneralReadout) -> String {
    let read = readout.uptime();
    let time = ok_readout(read);
    seconds_to_string(time)
}

pub fn fetch_cpu(readout: &GeneralReadout) -> String {
    let read = readout.cpu_model_name();
    ok_readout(read).trim().to_string()
}

pub fn fetch_memory(readout: &MemoryReadout) -> String {
    let read_total = readout.total();
    let total = ok_readout(read_total);
    let total = memory_to_gb(total);

    // let read_free = readout.free();
    // let free = ok_readout(read_free);
    // let free = memory_to_gb(free);

    let read_used = readout.used();
    let used = ok_readout(read_used);
    let used = memory_to_gb(used);

    let mem = Memory {
        total,
        /*free,*/
        used,
    };
    mem.to_string()
}

pub fn fetch_packages(readout: &PackageReadout) -> String {
    let read = readout.count_pkgs();
    let total: usize = read.iter().map(|(_, count)| *count).sum();
    total.to_string().trim().to_string()
}

pub fn fetch_gpu1(readout: &GeneralReadout) -> String {
    let read = readout.gpus();
    let v = ok_readout(read);
    String::from(&v[0])
}
pub fn fetch_gpu2(readout: &GeneralReadout) -> String {
    let read = readout.gpus();
    let v = ok_readout(read);
    String::from(&v[1])
}

pub fn fetch_user(readout: &GeneralReadout) -> String {
    let read = readout.username();
    ok_readout(read).trim().to_string()
}

pub fn fetch_terminal(readout: &GeneralReadout) -> String {
    let read = readout.terminal();
    ok_readout(read).trim().to_string()
}

// pub fn fetch_user() -> String {
// }

pub fn fetch(general: &GeneralReadout, opts: FetchOpts) -> Fetched {
    let mut fetched = Fetched::default();
    let mut opts = opts;
    if let Some(true) = opts.all {
        opts.os = Some(true);
        opts.os = Some(true);
        opts.host = Some(true);
        opts.kernel = Some(true);
        opts.user = Some(true);
        opts.uptime = Some(true);
        opts.cpu = Some(true);
        opts.memory = Some(true);
        opts.window_manager = Some(true);
        opts.session = Some(true);
        opts.terminal = Some(true);
        opts.packages = Some(true);
        opts.gpu1 = Some(true);
        opts.gpu2 = Some(true);
    }

    if let Some(true) = opts.os {
        let os = fetch_os(general);
        fetched.os = Some(os);
    }
    if let Some(true) = opts.kernel {
        let kernel_readout = KernelReadout::new();
        let kernel = fetch_kernel(&kernel_readout);
        fetched.kernel = Some(kernel);
    }
    if let Some(true) = opts.host {
        let host = fetch_host(general);
        fetched.host = Some(host);
    }
    if let Some(true) = opts.user {
        let user = fetch_user(general);
        fetched.user = Some(user);
    }

    if let Some(true) = opts.window_manager {
        let window_manager = fetch_window_manager(general);
        fetched.window_manager = Some(window_manager);
    }
    if let Some(true) = opts.session {
        let session = fetch_session(general);
        fetched.session = Some(session);
    }
    if let Some(true) = opts.terminal {
        let terminal = fetch_terminal(general);
        fetched.terminal = Some(terminal);
    }
    if let Some(true) = opts.uptime {
        let uptime = fetch_uptime(general);
        fetched.uptime = Some(uptime);
    }
    if let Some(true) = opts.cpu {
        let cpu = fetch_cpu(general);
        fetched.cpu = Some(cpu);
    }
    if let Some(true) = opts.memory {
        let memory_readout = MemoryReadout::new();
        let memory = fetch_memory(&memory_readout);
        fetched.memory = Some(memory);
    }
    if let Some(true) = opts.packages {
        let package_readout = PackageReadout::new();
        let packages = fetch_packages(&package_readout);
        fetched.packages = Some(packages);
    }

    fetched
}
