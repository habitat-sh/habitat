#![allow(dead_code)]

use crate::cli::{file_exists,
                 valid_origin};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
/// Commands relating to plans and other app-specific configuration
pub enum Plan {
    Init(PlanInit),
    Render(PlanRender),
}

/// Generates common package specific configuration files. Executing without argument will
/// create a `habitat` directory in your current folder for the plan. If `PKG_NAME` is
/// specified it will create a folder with that name. Environment variables (those starting
/// with 'pkg_') that are set will be used in the generated plan
#[derive(Parser)]
pub struct PlanInit {
    /// Name for the new app
    #[clap(name = "PKG_NAME")]
    pkg_name: Option<String>,

    /// Origin for the new app
    #[clap(name = "ORIGIN",
           short = "o",
        long = "origin",
        validator = valid_origin)]
    origin: Option<String>,

    /// Create a minimal plan file
    #[clap(name = "MIN", short = "m", long = "min")]
    min: bool,

    /// Specify explicit Scaffolding for your app (ex: node, ruby)
    #[clap(name = "SCAFFOLDING", short = "s", long = "scaffolding")]
    scaffolding: Option<String>,
}

/// Renders plan config files
#[derive(Parser)]
pub struct PlanRender {
    /// Path to default.toml
    #[clap(name = "DEFAULT_TOML",
           short = "d",
           long = "default-toml",
           default_value = "./default.toml")]
    default_toml: PathBuf,

    /// Path to config to render
    #[clap(name = "TEMPLATE_PATH", validator = file_exists)]
    template_path: PathBuf,

    /// Path to user.toml, defaults to none
    #[clap(name = "USER_TOML", short = "u", long = "user-toml")]
    user_toml: Option<PathBuf>,

    /// Path to json file with mock data for template, defaults to none
    #[clap(name = "MOCK_DATA", short = "m", long = "mock-data")]
    mock_data: Option<PathBuf>,

    /// Prints config to STDOUT
    #[clap(name = "PRINT", short = "p", long = "print")]
    print: bool,

    /// Path to render templates
    #[clap(name = "RENDER_DIR",
           short = "r",
           long = "render-dir",
           default_value = "./results")]
    render_dir: PathBuf,

    /// Don't write anything to disk, ignores --render-dir
    #[clap(name = "NO_RENDER", short = "n", long = "no-render")]
    no_render: bool,

    /// Don't print any helper messages.  When used with `--print` will only print config file
    #[clap(name = "QUIET", short = "q", long = "no-verbose", long = "quiet")]
    quiet: bool,
}
