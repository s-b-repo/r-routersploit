use anyhow::Result;
use crate::cli::Cli;
use crate::commands::{exploit, scanner, creds};

pub async fn handle_command(command: &str, cli_args: &Cli) -> Result<()> {
    match command {
        "exploit" => {
            let target = cli_args.target.clone().unwrap_or_default();
            let module = cli_args.module.clone().unwrap_or_default();
            exploit::run_exploit(&module, &target).await?;
        },
        "scanner" => {
            let target = cli_args.target.clone().unwrap_or_default();
            let module = cli_args.module.clone().unwrap_or_default();
            scanner::run_scan(&module, &target).await?;
        },
        "creds" => {
            let target = cli_args.target.clone().unwrap_or_default();
            let module = cli_args.module.clone().unwrap_or_default();
            creds::run_cred_check(&module, &target).await?;
        },
        _ => {
            eprintln!("Unknown command '{}'", command);
        }
    }
    Ok(())
}

// This function is also used by the shell:
pub async fn run_module(module_path: &str, target: &str) -> Result<()> {
    // Decide which "category" the module belongs to by the path
    // e.g. "exploits/sample_exploit" -> exploit::run_exploit(...)
    if module_path.starts_with("exploits/") {
        let module_name = module_path.trim_start_matches("exploits/").to_string();
        exploit::run_exploit(&module_name, target).await?;
    } else if module_path.starts_with("scanners/") {
        let module_name = module_path.trim_start_matches("scanners/").to_string();
        scanner::run_scan(&module_name, target).await?;
    } else if module_path.starts_with("creds/") {
        let module_name = module_path.trim_start_matches("creds/").to_string();
        creds::run_cred_check(&module_name, target).await?;
    } else {
        eprintln!("Unknown module path '{}'", module_path);
    }

    Ok(())
}
