// SigmaOS Update Manager CLI
use sigmaos_update_manager::{UpdateManager, FullUpdateStrategy, SecurityUpdateStrategy, LoggingObserver};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: sigmaos-update-manager <strategy>");
        println!("Strategies: full, security");
        return;
    }
    
    let strategy = &args[1];
    let mut manager = UpdateManager::new();
    manager.add_observer(Box::new(LoggingObserver));
    
    let result = match strategy.as_str() {
        "full" => manager.execute_update(Box::new(FullUpdateStrategy)),
        "security" => manager.execute_update(Box::new(SecurityUpdateStrategy)),
        _ => {
            eprintln!("Unknown strategy: {}", strategy);
            return;
        }
    };
    
    match result {
        Ok(_) => println!("Update completed successfully"),
        Err(e) => eprintln!("Update failed: {}", e),
    }
}
