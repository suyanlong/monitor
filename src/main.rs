#![allow(unused_imports, dead_code)]

extern crate getopts;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;
extern crate rand;

use rand::Rng;

use getopts::Options;
use prometheus::{Counter, CounterVec, GaugeVec, Histogram};
use std::env;
use std::thread;
use std::time;

lazy_static! {
	static ref PUSH_COUNTER: CounterVec = register_counter_vec!(
		"example_push_total",
		"Total number of prometheus client pushed.",
		&["thread","name"]
	).unwrap();

	static ref PUSH_REQ_HISTOGRAM: Histogram = register_histogram!(
		"example_push_request_duration_seconds",
		"The push request latencies in seconds."
	).unwrap();

	static ref PUSH_RAND_VAULE: GaugeVec = register_gauge_vec!{
		"push_rand_value",
		"the push rand value",
		&["name","node"]
	}.unwrap();
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut rng = rand::thread_rng();
    let mut opts = Options::new();
    opts.optflagopt(
        "A",
        "addr",
        "prometheus pushgateway address",
        "default 127.0.0.1:9091",
    );
    opts.optflag("h", "help", "print this help menu");
    println!("{:?}", args);

    let matches = opts.parse(&args).unwrap();
    if matches.opt_present("h") || !matches.opt_present("A") {
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(&brief));
        return;
    }
    println!("Pushing, please start Pushgateway first.");
    let address = matches.opt_str("A").unwrap_or("127.0.0.1:9091".to_owned());
    println!("{:?}", address);
    for _ in 0..5000 {
        let _timer = PUSH_REQ_HISTOGRAM.start_timer(); // drop as observe
        thread::sleep(time::Duration::from_secs(1));
        let thread = format!("{:?}", thread::current().id());
        PUSH_COUNTER
            .with_label_values(&[thread.as_str(), "main"])
            .inc();
        PUSH_COUNTER
            .with_label_values(&[thread.as_str(), "main1"])
            .inc();
        PUSH_COUNTER
            .with_label_values(&[thread.as_str(), "main2"])
            .inc();
        let value = rng.gen_range(1, 100);
        println!("{:?}",value);
        PUSH_RAND_VAULE.with_label_values(&[thread.as_str(), "main3"]).add(value as f64);

        let metric_familys = prometheus::gather();
        _timer.observe_duration();
        prometheus::push_metrics(
            "example_push",
            labels! {"instance".to_owned() => "HAL-9000".to_owned(),},
            &address,
            metric_familys,
        ).unwrap();
    }

    println!("Okay, please check the Pushgateway.");
}
