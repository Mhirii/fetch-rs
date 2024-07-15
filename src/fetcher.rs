extern crate os_release;
use crate::utils;
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
    free: String,
    used: String,
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for Memory {
    fn to_string(&self) -> String {
        format!("{} GB / {} GB", self.used, self.total)
    }
}

pub fn fetch_os(readout: &GeneralReadout) -> String {
    let read = readout.distribution();
    utils::ok_readout(read).trim().to_string()
}

pub fn fetch_kernel(kernel_readout: &KernelReadout) -> String {
    let read = kernel_readout.os_release();
    utils::ok_readout(read).trim().to_string()
}

pub fn fetch_host(readout: &GeneralReadout) -> String {
    let read = readout.hostname();
    utils::ok_readout(read).trim().to_string()
}

pub fn fetch_window_manager(readout: &GeneralReadout) -> String {
    let read = readout.window_manager();
    utils::ok_readout(read).trim().to_string()
}

pub fn fetch_session(readout: &GeneralReadout) -> String {
    let read = readout.session();
    utils::ok_readout(read).trim().to_string()
}

pub fn fetch_uptime(readout: &GeneralReadout) -> String {
    let read = readout.uptime();
    let time = utils::ok_readout(read);
    utils::seconds_to_string(time)
}

pub fn fetch_cpu(readout: &GeneralReadout) -> String {
    let read = readout.cpu_model_name();
    utils::ok_readout(read).trim().to_string()
}

pub fn fetch_memory(readout: &MemoryReadout) -> String {
    let read_total = readout.total();
    let total = utils::ok_readout(read_total);
    let total = utils::memory_to_gb(total);

    let read_free = readout.free();
    let free = utils::ok_readout(read_free);
    let free = utils::memory_to_gb(free);

    let read_used = readout.used();
    let used = utils::ok_readout(read_used);
    let used = utils::memory_to_gb(used);

    let mem = Memory { total, free, used };
    mem.to_string()
}

pub fn fetch_packages(readout: &PackageReadout) -> String {
    let read = readout.count_pkgs();
    let total: usize = read.iter().map(|(_, count)| *count).sum();
    total.to_string().trim().to_string()
}

pub fn fetch_user(readout: &GeneralReadout) -> String {
    let read = readout.username();
    utils::ok_readout(read).trim().to_string()
}

pub fn fetch_terminal(readout: &GeneralReadout) -> String {
    let read = readout.terminal();
    utils::ok_readout(read).trim().to_string()
}

// pub fn fetch_user() -> String {
// }

pub fn fetch(opts: FetchOpts) -> Fetched {
    let general_readout = GeneralReadout::new();
    let general = &general_readout;

    let mut fetched = Fetched::default();

    if opts.os.unwrap_or(true) {
        let os = fetch_os(general);
        fetched.os = Some(os);
    }
    if opts.kernel.unwrap_or(true) {
        let kernel_readout = KernelReadout::new();
        let kernel = fetch_kernel(&kernel_readout);
        fetched.kernel = Some(kernel);
    }
    if opts.host.unwrap_or(true) {
        let host = fetch_host(general);
        fetched.host = Some(host);
    }
    if opts.user.unwrap_or(true) {
        let user = fetch_user(general);
        fetched.user = Some(user);
    }

    if opts.window_manager.unwrap_or(true) {
        let window_manager = fetch_window_manager(general);
        fetched.window_manager = Some(window_manager);
    }
    if opts.session.unwrap_or(true) {
        let session = fetch_session(general);
        fetched.session = Some(session);
    }
    if opts.terminal.unwrap_or(true) {
        let terminal = fetch_terminal(general);
        fetched.terminal = Some(terminal);
    }
    if opts.uptime.unwrap_or(true) {
        let uptime = fetch_uptime(general);
        fetched.uptime = Some(uptime);
    }
    if opts.cpu.unwrap_or(true) {
        let cpu = fetch_cpu(general);
        fetched.cpu = Some(cpu);
    }
    if opts.memory.unwrap_or(true) {
        let memory_readout = MemoryReadout::new();
        let memory = fetch_memory(&memory_readout);
        fetched.memory = Some(memory);
    }
    if opts.packages.unwrap_or(true) {
        let package_readout = PackageReadout::new();
        let packages = fetch_packages(&package_readout);
        fetched.packages = Some(packages);
    }

    fetched
}
