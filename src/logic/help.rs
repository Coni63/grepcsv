use crate::structs::config::Config;

pub fn print_usage(config: &Config) {
    let program = &config.program;
    let opts = Config::get_opts();

    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
