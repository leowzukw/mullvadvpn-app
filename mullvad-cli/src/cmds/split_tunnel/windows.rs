use crate::{new_rpc_client, Command, Result};
use clap::value_t_or_exit;

pub struct SplitTunnel;

impl Command for SplitTunnel {
    fn name(&self) -> &'static str {
        "split-tunnel"
    }

    fn clap_subcommand(&self) -> clap::App<'static, 'static> {
        clap::SubCommand::with_name(self.name())
            .about("Set options for applications to exclude from the tunnel")
            .setting(clap::AppSettings::SubcommandRequiredElseHelp)
            .subcommand(create_app_subcommand())
            .subcommand(
                clap::SubCommand::with_name("set")
                    .about("Enable or disable split tunnel")
                    .arg(
                        clap::Arg::with_name("policy")
                            .required(true)
                            .possible_values(&["on", "off"]),
                    ),
            )
            .subcommand(clap::SubCommand::with_name("get").about("Display the split tunnel status"))
    }

    fn run(&self, matches: &clap::ArgMatches<'_>) -> Result<()> {
        match matches.subcommand() {
            ("app", Some(matches)) => Self::handle_app_subcommand(matches),
            ("get", _) => self.get(),
            ("set", Some(matches)) => {
                let enabled = value_t_or_exit!(matches.value_of("policy"), String);
                self.set(enabled == "on")
            }
            _ => {
                unreachable!("unhandled command");
            }
        }
    }
}

fn create_app_subcommand() -> clap::App<'static, 'static> {
    clap::SubCommand::with_name("app")
        .about("Manage applications to exclude from the tunnel")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(clap::SubCommand::with_name("list"))
        .subcommand(
            clap::SubCommand::with_name("add").arg(clap::Arg::with_name("path").required(true)),
        )
        .subcommand(
            clap::SubCommand::with_name("remove").arg(clap::Arg::with_name("path").required(true)),
        )
}

impl SplitTunnel {
    fn handle_app_subcommand(matches: &clap::ArgMatches<'_>) -> Result<()> {
        match matches.subcommand() {
            ("list", Some(_)) => {
                let paths = new_rpc_client()?.get_split_tunnel_apps()?;

                println!("Excluded applications:");
                for path in paths {
                    println!("    {}", path);
                }

                Ok(())
            }
            ("add", Some(matches)) => {
                let path = value_t_or_exit!(matches.value_of("path"), String);
                new_rpc_client()?.add_split_tunnel_app(path)?;
                Ok(())
            }
            ("remove", Some(matches)) => {
                let path = value_t_or_exit!(matches.value_of("path"), String);
                new_rpc_client()?.remove_split_tunnel_app(path)?;
                Ok(())
            }
            _ => unreachable!("unhandled subcommand"),
        }
    }

    fn set(&self, enabled: bool) -> Result<()> {
        let mut rpc = new_rpc_client()?;
        rpc.set_split_tunnel_status(enabled)?;
        println!("Changed split tunnel setting");
        Ok(())
    }

    fn get(&self) -> Result<()> {
        let mut rpc = new_rpc_client()?;
        let enabled = rpc.get_settings()?.enable_exclusions;
        println!(
            "Split tunnel status: {}",
            if enabled { "on" } else { "off" }
        );
        Ok(())
    }
}
