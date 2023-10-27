mod args;
mod config;
mod tmux;

fn main() {
    let cli_args = args::get_options();
    let config_file = config::get_config();

    // Yes this processes some things twice, but by doing so,
    // it allows the program to fail quickly if options are incorrect
    validate_inputs(&cli_args, &config_file);

    // TODO: need a better name for reconciliation of config file & cli args
    let configuration = get_configuration(&cli_args, &config_file);

    launch(configuration, &config_file);
}

fn validate_inputs(cli_args: &args::LaunchArgs, config_file: &config::Config) {
    if cli_args.preset.is_none() && cli_args.windows.is_none() {
        panic!("Please specify a preset or list of windows to launch");
    }
    if cli_args.preset.is_some() {
        let preset_name = cli_args.preset.clone().unwrap();
        if !config_file.presets.contains_key(&preset_name) {
            panic!("There is no preset with the name {}", preset_name)
        }
    } else {
        if cli_args.windows.is_some() {
            for w in cli_args.windows.clone().unwrap().split([' ', '+']) {
                if !config_file.profiles.contains_key(w) {
                    panic!("There is no profile with the name {}", w);
                }
            }
        }
    }
}

fn get_configuration(cli_args: &args::LaunchArgs, config_file: &config::Config) -> config::Preset {
    if cli_args.preset.is_some() {
        let preset_name = cli_args.preset.clone().unwrap();
        config_file.get_preset(preset_name)
    } else {
        config::Preset {
            name: cli_args.name.clone().unwrap_or("dev".to_string()),
            windows: cli_args.windows
                .clone()
                .unwrap()
                .split(" ")
                .map(|w| w.to_string())
                .collect::<Vec<_>>(),
            fresh: cli_args.fresh
                .split(" ")
                .map(|f| f.to_string())
                .collect::<Vec<_>>(),
        }
    }
}

fn launch(configuration: config::Preset, config_file: &config::Config) {
    //> Start tmux session
    tmux::cmd_start_session(&configuration.name);

    for w in configuration.windows.iter() {
        let window = w.clone();
        let panes = window.split("+").collect::<Vec<_>>();

        for (i, pane_name) in panes.iter().enumerate() {
            let profile = config_file.get_profile(pane_name);
            let target = format!("{}:{}.{}", configuration.name, panes[0], i + 1);
            let run_fresh = configuration.fresh.contains(&pane_name.to_string());

            //> create window or pane
            if i < 1 {
                tmux::cmd_new_window(&pane_name.to_string());
            } else {
                tmux::cmd_split_window();
            }

            //> set directory
            tmux::cmd_set_dir(&target, &profile.dir);

            //> pull develop
            if run_fresh {
                for cmd in &config_file.fresh_cmds {
                    tmux::cmd_run_cmd(&target, &cmd);
                }
            }

            //> run commands
            for cmd in profile.cmds {
                tmux::cmd_run_cmd(&target, &cmd);
            }
        }
    }

    //> remove initial window
    tmux::cmd_cleanup(&configuration.name);

    //> attach to session
    tmux::cmd_attach();
}
