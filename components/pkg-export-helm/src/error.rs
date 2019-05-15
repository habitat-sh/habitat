#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to launch `helm` command. Please ensure Helm is installed.")]
    HelmLaunchFailed,
    #[fail(display = "{}. Please ensure Helm is initialized. For only setting up the Helm \
                      client, please run `helm init -c`.",
           _0)]
    HelmNotSetup(String),
    #[fail(display = "Invalid maintainer specification '{}', must be of the form \
                      NAME[,EMAIL[,URL]]",
           _0)]
    InvalidMaintainer(String),
    #[fail(display = "Invalid URL '{}': {}", _0, _1)]
    InvalidUrl(String, String),
}
