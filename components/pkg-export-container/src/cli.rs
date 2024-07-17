use crate::{engine,
            HabHartIdParser,
            RegistryType,
            UrlValueParser};
use clap::{builder::Str,
           value_parser,
           Arg,
           ArgAction,
           Command};
use habitat_common::PROGRAM_NAME;
use habitat_core::url::default_bldr_url;

/// The version of this library and program when built.
const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

/// Create the Clap CLI for the container exporter
pub fn cli() -> Command {
    let name: &str = &PROGRAM_NAME;
    let about = "Creates a container image from a set of Habitat packages (and optionally pushes \
                 to a remote repository)";

    let cmd =
        Command::new(name).max_term_width(80)
                          .about(about)
                          .version(VERSION)
                          .author("\nAuthors: The Habitat Maintainers <humans@habitat.sh>")
                          .help_template("{name} {version} {author-section} {about-section} \
                                          \n{usage-heading} {usage}\n\n{all-args}")
                          .arg(Arg::new("IMAGE_NAME").long("image-name")
                                                     .short('i')
                                                     .value_name("IMAGE_NAME")
                                                     .help("Image name template: supports: \
                                                            {{pkg_origin}}/{{pkg_name}} \
                                                            {{pkg_origin}}, {{pkg_name}}, \
                                                            {{pkg_version}}, {{pkg_release}}, \
                                                            {{channel}})")
                                                     .default_value("{{pkg_origin/pkg_name}}"));

    let cmd = add_base_packages_args(cmd);
    let cmd = add_builder_args(cmd);
    let cmd = add_tagging_args(cmd);
    let cmd = add_publishing_args(cmd);
    let cmd = add_memory_arg(cmd);
    let cmd = add_layer_arg(cmd);
    let cmd = add_pkg_ident_arg(cmd);
    let cmd = add_engine_arg(cmd);

    if cfg!(windows) {
        add_base_image_arg(cmd)
    } else {
        cmd
    }
}

fn add_base_packages_args(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("HAB_PKG")
            .long("hab-pkg")
            .value_name("HAB_PKG")
            .default_value(super::DEFAULT_HAB_IDENT)
            .value_parser(HabHartIdParser)
            .help(
                "Habitat CLI package identifier (ex: acme/redis) or filepath to a Habitat \
                         artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart) \
                         to install",
            ),
    )
    .arg(
        Arg::new("HAB_LAUNCHER_PKG")
            .long("launcher-pkg")
            .value_name("HAB_LAUNCHER_PKG")
            .default_value(super::DEFAULT_LAUNCHER_IDENT)
            .value_parser(HabHartIdParser)
            .help(
                "Launcher package identifier (ex: core/hab-launcher) or filepath to a \
                         Habitat artifact (ex: \
                         /home/core-hab-launcher-13829-20200527165030-x86_64-linux.hart) to \
                         install",
            ),
    )
    .arg(
        Arg::new("HAB_SUP_PKG")
            .long("sup-pkg")
            .value_name("HAB_SUP_PKG")
            .default_value(super::DEFAULT_SUP_IDENT)
            .value_parser(HabHartIdParser)
            .help(
                "Supervisor package identifier (ex: core/hab-sup) or filepath to a \
                 Habitat artifact (ex: \
                 /home/core-hab-sup-1.6.39-20200527165021-x86_64-linux.hart) to install",
            ),
    )
}

fn add_builder_args(cmd: Command) -> Command {
    cmd.arg(Arg::new("BLDR_URL").long("url")
                                .short('u')
                                .value_name("BLDR_URL")
                                .default_value(Into::<Str>::into(default_bldr_url()))
                                .value_parser(UrlValueParser)
                                .help("Install packages from Builder at the specified URL"))
       .arg(Arg::new("CHANNEL").long("channel")
                               .short('c')
                               .value_name("CHANNEL")
                               .default_value("stable")
                               .help("Install packages from the specified release channel"))
       .arg(Arg::new("BASE_PKGS_BLDR_URL").long("base-pkgs-url")
                                          .value_name("BASE_PKGS_BLDR_URL")
                                          .default_value(Into::<Str>::into(default_bldr_url()))
                                          .value_parser(UrlValueParser)
                                          .help("Install base packages from Builder at the \
                                                 specified URL"))
       .arg(Arg::new("BASE_PKGS_CHANNEL").long("base-pkgs-channel")
                                         .value_name("BASE_PKGS_CHANNEL")
                                         .default_value("stable")
                                         .help("Install base packages from the specified release"))
       .arg(Arg::new("BLDR_AUTH_TOKEN").long("auth")
                                       .short('z')
                                       .value_name("BLDR_AUTH_TOKEN")
                                       .help("Provide a Builder auth token for private pkg export"))
}

fn add_tagging_args(cmd: Command) -> Command {
    cmd.arg(Arg::new("TAG_VERSION_RELEASE").long("tag-version-release")
                                           .conflicts_with("NO_TAG_VERSION_RELEASE")
                                           .action(ArgAction::SetTrue)
                                           .help("Tag image with \
                                                  :\"{{pkg_version}}-{{pkg_release}}\""))
       .arg(Arg::new("NO_TAG_VERSION_RELEASE").long("no-tag-version-release")
                                              .conflicts_with("TAG_VERSION_RELEASE")
                                              .action(ArgAction::SetTrue)
                                              .help("Do not tag image with \
                                                     :\"{{pkg_version}}-{{pkg_release}}\""))
       .arg(Arg::new("TAG_VERSION").long("tag-version")
                                   .conflicts_with("NO_TAG_VERSION")
                                   .action(ArgAction::SetTrue)
                                   .help("Tag image with :\"{{pkg_version}}\""))
       .arg(Arg::new("NO_TAG_VERSION").long("no-tag-version")
                                      .conflicts_with("TAG_VERSION")
                                      .action(ArgAction::SetTrue)
                                      .help("Do not tag image with :\"{{pkg_version}}\""))
       .arg(Arg::new("TAG_LATEST").long("tag-latest")
                                  .conflicts_with("NO_TAG_LATEST")
                                  .action(ArgAction::SetTrue)
                                  .help("Tag image with :\"latest\""))
       .arg(Arg::new("NO_TAG_LATEST").long("no-tag-latest")
                                     .conflicts_with("TAG_LATEST")
                                     .action(ArgAction::SetTrue)
                                     .help("Do not tag image with :\"latest\""))
       .arg(Arg::new("TAG_CUSTOM").long("tag-custom")
                                  .value_name("TAG_CUSTOM")
                                  .help("Tag image with additional custom tag (supports: \
                                         {{pkg_origin}}, {{pkg_name}}, {{pkg_version}}, \
                                         {{pkg_release}}, {{channel}})"))
}

fn add_publishing_args(cmd: Command) -> Command {
    cmd.arg(Arg::new("PUSH_IMAGE")
                    .long("push-image")
                    .conflicts_with("NO_PUSH_IMAGE")
                    .requires_all(["REGISTRY_USERNAME", "REGISTRY_PASSWORD"])
                    .action(ArgAction::SetTrue)
                    .help("Push image to remote registry (default: no)"),
            )
            .arg(
                Arg::new("NO_PUSH_IMAGE")
                    .long("no-push-image")
                    .conflicts_with("PUSH_IMAGE")
                    .action(ArgAction::SetTrue)
                    .help("Do not push image to remote registry (default: yes)"),
            )
            .arg(
                Arg::new("REGISTRY_USERNAME")
                    .long("username")
                    .short('U')
                    .value_name("REGISTRY_USERNAME")
                    .requires("REGISTRY_PASSWORD")
                    .help(
                        "Remote registry username, required for pushing image to remote registry",
                    ),
            )
            .arg(
                Arg::new("REGISTRY_PASSWORD")
                    .long("password")
                    .short('P')
                    .value_name("REGISTRY_PASSWORD")
                    .requires("REGISTRY_USERNAME")
                    .help(
                        "Remote registry password, required for pushing image to remote registry",
                    ),
            )
            .arg(
                Arg::new("REGISTRY_TYPE")
                    .value_parser(value_parser!(RegistryType))
                    .long("registry-type")
                    .short('R')
                    .value_name("REGISTRY_TYPE")
                    .default_value("docker")
                    .help("Remote registry type (default: docker)"),
            )
            .arg(
                Arg::new("REGISTRY_URL")
                    // This is not strictly a requirement but will keep someone from
                    // making a mistake when inputing an ECR URL
                    .required_if_eq("REGISTRY_TYPE", "amazon")
                    .required_if_eq("REGISTRY_TYPE", "azure")
                    .long("registry-url")
                    .short('G')
                    .value_name("REGISTRY_URL")
                    .help("Remote registry url"),
            )
            // Cleanup
            .arg(
                Arg::new("RM_IMAGE")
                    .long("rm-image")
                    .action(ArgAction::SetTrue)
                    .help("Remove local image from engine after build and/or push (default: no)"),
            )
}

fn add_pkg_ident_arg(cmd: Command) -> Command {
    let help = "One or more Habitat package identifiers (ex: acme/redis) and/or filepaths to a \
                Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)";

    cmd.arg(Arg::new("PKG_IDENT_OR_ARTIFACT").value_name("PKG_IDENT_OR_ARTIFACT")
                                             .required(true)
                                             .num_args(1..)
                                             .help(help))
}

fn add_memory_arg(cmd: Command) -> Command {
    cmd.arg(Arg::new("MEMORY_LIMIT").value_name("MEMORY_LIMIT")
                                    .long("memory")
                                    .short('m')
                                    .help("Memory limit passed to docker build's --memory arg \
                                           (ex: 2gb)"))
}

fn add_base_image_arg(cmd: Command) -> Command {
    cmd.arg(Arg::new("BASE_IMAGE").value_name("BASE_IMAGE")
                                  .long("base-image")
                                  .help("Base image of the final exported image --base-image \
                                         mcr.microsoft.com/windows/servercore:ltsc2019"))
}

fn add_layer_arg(cmd: Command) -> Command {
    cmd.arg(Arg::new("MULTI_LAYER").value_name("MULTI_LAYER")
                                   .long("multi-layer")
                                   .required(false)
                                   .action(ArgAction::SetTrue)
                                   .help("If specified, creates an image where each Habitat \
                                          package is added in its own layer, in dependency order \
                                          (that is, low-level dependencies are added first, with \
                                          user packages added last). This will allow for \
                                          reusable layers, reducing storage and network \
                                          transmission costs. If the resulting image cannot be \
                                          built because there are too many layers, re-build \
                                          without specifying this option to add all Habitat \
                                          packages in a single layer (which is the default \
                                          behavior)."))
}

fn add_engine_arg(cmd: Command) -> Command {
    let arg = engine::cli_arg();
    cmd.arg(arg)
}
