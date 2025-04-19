use clap::{App, Arg};
pub struct Config {
    pub url: String,
    pub database: String,
    pub spoof_ip: String,
    pub user_agent: String,
    pub crawl_subs: bool,
    pub crawl_everything: bool,
    pub throttle_secs: u64,
}
impl Config {
    pub fn parse() -> Self {
        let matches = App::new("Lockjaw Spider")
            .version("2.0")
            .author("By Mephistolist")
            .about("Web spider in Rust that helps to hide tracks.")
            .arg(Arg::with_name("url")
                .short("u")
                .long("url")
                .value_name("URL")
                .takes_value(true)
                .required(true)
                .help("Sets the starting URL for the spider"))
            .arg(Arg::with_name("database")
                .short("d")
                .long("database")
                .value_name("DB_FILE")
                .takes_value(true)
                .required(true)
                .help("Sets the SQLite database file"))
            .arg(Arg::with_name("spoof")
                .short("x")
                .long("spoof")
                .value_name("Spoofed_IP")
                .takes_value(true)
                .help("Sets the spoofed IP for headers"))
            .arg(Arg::with_name("user_agent")
                .short("a")
                .long("user-agent")
                .value_name("USER_AGENT")
                .takes_value(true)
                .default_value("Lockjaw Spider 2.0")
                .help("Sets the user agent string"))
            .arg(Arg::with_name("subs")
                .short("s")
                .long("subs")
                .takes_value(false)
                .help("Enable crawling subdomains"))
            .arg(Arg::with_name("everything")
                .short("e")
                .long("everything")
                .takes_value(false)
                .help("Spider all URLs but only follow same-domain or subdomain links"))
            .arg(Arg::with_name("time")
                .short("t")
                .long("time")
                .value_name("SECONDS")
                .takes_value(true)
                .default_value("1")
                .validator(|v| match v.parse::<u64>() {
                    Ok(n) if n >= 1 && n <= 300 => Ok(()),
                    _ => Err(String::from("Must be a positive number between 1 and 300")),
                })
                .help("Delay between requests (1-300 seconds)"))
            .get_matches();
        Config {
            url: matches.value_of("url").unwrap().to_string(),
            database: matches.value_of("database").unwrap().to_string(),
            spoof_ip: matches.value_of("spoof").unwrap_or("").to_string(),
            user_agent: matches.value_of("user_agent").unwrap().to_string(),
            crawl_subs: matches.is_present("subs"),
            crawl_everything: matches.is_present("everything"),
            throttle_secs: matches.value_of("time").unwrap().parse().unwrap(),
        }
    }
}
