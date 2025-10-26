use pulsehome::HomeHub;
use pulsehome::cli::CLI;
use pulsehome::observer::display_observer::DisplayObserver;
use pulsehome::observer::logger_observer::LoggerObserver;

fn main() {
    let mut hub = HomeHub::new();

    // Register observers
    hub.register_observer(Box::new(DisplayObserver::new()));
    hub.register_observer(Box::new(LoggerObserver::new("home_log.txt")));

    // Start CLI
    let mut cli = CLI::new(hub);
    cli.start();
}
