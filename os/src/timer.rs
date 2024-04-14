use loongarch::{register::{tcfg, cpucfg::Cpucfg}, time};

pub fn init_trigger(ticks_per_sec: usize) {
    let stable_counter_freq = Cpucfg::get_sc_freq();
    let trigger_freq = stable_counter_freq / ticks_per_sec;
    let mut tcfg = tcfg::read();
    tcfg.init_trigger(trigger_freq);
}

pub fn get_time_s() -> usize {
    let stable_counter_freq = Cpucfg::get_sc_freq();
    get_time() / stable_counter_freq
}

pub fn get_time_ms() -> usize {
    let stable_counter_freq = Cpucfg::get_sc_freq();
    get_time() * 1_000 / stable_counter_freq
}

pub fn get_time_us() -> usize {
    let stable_counter_freq = Cpucfg::get_sc_freq();
    get_time() * 1_000_000 / stable_counter_freq
}

pub fn get_time() -> usize {
    time::get_time()
}
