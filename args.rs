use argh::FromArgs;

#[derive(FromArgs)]
/// Launch tmux dev environment
pub struct LaunchArgs {
    /// specifies a preset in the config file
    #[argh(option, short = 'p')]
    pub preset: Option<String>,

    /// specifies session name
    #[argh(option, short = 'n')]
    pub name: Option<String>,

    /// space separated list of apps to run in separate windows
    #[argh(option, short = 'w')]
    pub windows: Option<String>,

    /// specifies which apps should pull latest develop branch
    /// before starting
    #[argh(option, short = 'f', default = "String::from(\"\")")]
    pub fresh: String,
}

pub fn get_options() -> LaunchArgs {
    return argh::from_env();
}
