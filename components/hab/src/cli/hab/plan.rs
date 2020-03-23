use crate::cli::{file_exists,
                 valid_origin};
use configopt::ConfigOpt;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to plans and other app-specific configuration
pub enum Plan {
    /// Generates common package specific configuration files. Executing without argument will
    /// create a `habitat` directory in your current folder for the plan. If `PKG_NAME` is
    /// specified it will create a folder with that name. Environment variables (those starting
    /// with 'pkg_') that are set will be used in the generated plan
    Init {
        /// Name for the new app
        #[structopt(name = "PKG_NAME")]
        pkg_name:    Option<String>,
        /// Origin for the new app
        #[structopt(name = "ORIGIN",
        short = "o",
        long = "origin",
        validator = valid_origin)]
        origin:      Option<String>,
        /// Create a minimal plan file
        #[structopt(name = "MIN", short = "m", long = "min")]
        min:         bool,
        /// Specify explicit Scaffolding for your app (ex: node, ruby)
        #[structopt(name = "SCAFFOLDING", short = "s", long = "scaffolding")]
        scaffolding: Option<String>,
    },
    /// Renders plan config files      
    Render {
        /// Path to default.toml
        #[structopt(name = "DEFAULT_TOML",
                    short = "d",
                    long = "default-toml",
                    default_value = "./default.toml")]
        default_toml:  PathBuf,
        /// Path to config to render
        #[structopt(name = "TEMPLATE_PATH", validator = file_exists)]
        template_path: PathBuf,
        /// Path to user.toml, defaults to none
        #[structopt(name = "USER_TOML", short = "u", long = "user-toml")]
        user_toml:     Option<PathBuf>,
        /// Path to json file with mock data for template, defaults to none        
        #[structopt(name = "MOCK_DATA", short = "m", long = "mock-data")]
        mock_data:     Option<PathBuf>,
        /// Prints config to STDOUT
        #[structopt(name = "PRINT", short = "p", long = "print")]
        print:         bool,
        /// Path to render templates        
        #[structopt(name = "RENDER_DIR",
                    short = "r",
                    long = "render-dir",
                    default_value = "./results")]
        render_dir:    PathBuf,
        /// Don't write anything to disk, ignores --render-dir        
        #[structopt(name = "NO_RENDER", short = "n", long = "no-render")]
        no_render:     bool,
        /// Don't print any helper messages.  When used with `--print` will only print config file
        #[structopt(name = "QUIET", short = "q", long = "no-verbose", long = "quiet")]
        quiet:         bool,
    },
}
