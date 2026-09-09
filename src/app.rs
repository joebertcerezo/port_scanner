use std::time::{Duration, Instant};

use crate::{args::cli_args, config::Config, error::AppError, scanner::ScanEngine};

pub struct App;

impl App {
    pub async fn run() -> Result<(), AppError> {
        let cli = cli_args().run();

        let config = Config {
            address: cli.address,
            start_port: cli.start_port,
            end_port: cli.end_port,
            concurrency: cli.concurrency,
            timeout: Duration::from_millis(cli.timeout_ms),
        };

        config.validate()?;

        println!("Port Scanner");
        println!("──────────────────────────────────────────────────");
        println!("Target       {}", config.address);
        println!("Ports        {}-{}", config.start_port, config.end_port);
        println!("Concurrency  {}", config.concurrency);
        println!("Timeout      {}ms", config.timeout.as_millis());
        println!("──────────────────────────────────────────────────");

        let start_time = Instant::now();
        let engine = ScanEngine::new(config);
        let open_ports = engine.run().await;
        let elapsed = start_time.elapsed();

        println!("\nScan finished in {:.2?}\n", elapsed);
        println!("{:<15} {:<10} {:<10} {:<15}", "TARGET", "PORT", "STATE", "SERVICE");
        println!("{:-<55}", "");

        if open_ports.is_empty() {
            println!("No open ports found in specified range.");
        } else {
            for res in open_ports {
                println!(
                    "{:<15} {:<10} {:<10} {:<15}",
                    res.address, res.port, res.state, res.service
                );
            }
        }

        Ok(())
    }
}
