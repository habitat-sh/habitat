use super::{svc::{Load,
                  Svc,
                  Update},
            Hab};
use crate::cli;
use clap::{App,
           AppSettings,
           ArgSettings};
use habitat_common::FeatureFlag;
use std::str;

fn feature_flags_for_cli_test() -> FeatureFlag {
    let mut f = FeatureFlag::empty();
    // Inorder for the `clap` and `structopt` comparison we must turn on this feature flag because
    // the `structopt` version does not have a way to conditionally add subcommands at runtime.
    f.insert(FeatureFlag::SERVICE_CONFIG_FILES);
    f
}

fn feature_flags_for_cli_test_with_structopt() -> FeatureFlag {
    let mut f = feature_flags_for_cli_test();
    f.insert(FeatureFlag::STRUCTOPT_CLI);
    f
}

fn help(app: &App) -> String {
    let mut help = Vec::new();
    app.write_help(&mut help).expect("to write help");
    String::from(str::from_utf8(&help).expect("to convert help to utf8"))
}

macro_rules! compare_app_flags {
    ($flags1:expr, $flags2:expr) => {
        assert_eq!($flags1.is_set(AppSettings::ArgsNegateSubcommands),
                   $flags2.is_set(AppSettings::ArgsNegateSubcommands),
                   "ArgsNegateSubcommands");
        assert_eq!($flags1.is_set(AppSettings::AllArgsOverrideSelf),
                   $flags2.is_set(AppSettings::AllArgsOverrideSelf),
                   "AllArgsOverrideSelf");
        assert_eq!($flags1.is_set(AppSettings::AllowExternalSubcommands),
                   $flags2.is_set(AppSettings::AllowExternalSubcommands),
                   "AllowExternalSubcommands");
        assert_eq!($flags1.is_set(AppSettings::AllowInvalidUtf8),
                   $flags2.is_set(AppSettings::AllowInvalidUtf8),
                   "AllowInvalidUtf8");
        assert_eq!($flags1.is_set(AppSettings::AllowLeadingHyphen),
                   $flags2.is_set(AppSettings::AllowLeadingHyphen),
                   "AllowLeadingHyphen");
        assert_eq!($flags1.is_set(AppSettings::AllowNegativeNumbers),
                   $flags2.is_set(AppSettings::AllowNegativeNumbers),
                   "AllowNegativeNumbers");
        assert_eq!($flags1.is_set(AppSettings::AllowMissingPositional),
                   $flags2.is_set(AppSettings::AllowMissingPositional),
                   "AllowMissingPositional");
        assert_eq!($flags1.is_set(AppSettings::ColoredHelp),
                   $flags2.is_set(AppSettings::ColoredHelp),
                   "ColoredHelp");
        assert_eq!($flags1.is_set(AppSettings::ColorAlways),
                   $flags2.is_set(AppSettings::ColorAlways),
                   "ColorAlways");
        assert_eq!($flags1.is_set(AppSettings::ColorAuto),
                   $flags2.is_set(AppSettings::ColorAuto),
                   "ColorAuto");
        assert_eq!($flags1.is_set(AppSettings::ColorNever),
                   $flags2.is_set(AppSettings::ColorNever),
                   "ColorNever");
        assert_eq!($flags1.is_set(AppSettings::DontDelimitTrailingValues),
                   $flags2.is_set(AppSettings::DontDelimitTrailingValues),
                   "DontDelimitTrailingValues");
        assert_eq!($flags1.is_set(AppSettings::DontCollapseArgsInUsage),
                   $flags2.is_set(AppSettings::DontCollapseArgsInUsage),
                   "DontCollapseArgsInUsage");
        assert_eq!($flags1.is_set(AppSettings::DeriveDisplayOrder),
                   $flags2.is_set(AppSettings::DeriveDisplayOrder),
                   "DeriveDisplayOrder");
        assert_eq!($flags1.is_set(AppSettings::DisableHelpFlags),
                   $flags2.is_set(AppSettings::DisableHelpFlags),
                   "DisableHelpFlags");
        assert_eq!($flags1.is_set(AppSettings::DisableHelpSubcommand),
                   $flags2.is_set(AppSettings::DisableHelpSubcommand),
                   "DisableHelpSubcommand");
        assert_eq!($flags1.is_set(AppSettings::DisableVersion),
                   $flags2.is_set(AppSettings::DisableVersion),
                   "DisableVersion");
        assert_eq!($flags1.is_set(AppSettings::GlobalVersion),
                   $flags2.is_set(AppSettings::GlobalVersion),
                   "GlobalVersion");
        assert_eq!($flags1.is_set(AppSettings::HidePossibleValuesInHelp),
                   $flags2.is_set(AppSettings::HidePossibleValuesInHelp),
                   "HidePossibleValuesInHelp");
        assert_eq!($flags1.is_set(AppSettings::Hidden),
                   $flags2.is_set(AppSettings::Hidden),
                   "Hidden");
        assert_eq!($flags1.is_set(AppSettings::LowIndexMultiplePositional),
                   $flags2.is_set(AppSettings::LowIndexMultiplePositional),
                   "LowIndexMultiplePositional");
        assert_eq!($flags1.is_set(AppSettings::NeedsLongHelp),
                   $flags2.is_set(AppSettings::NeedsLongHelp),
                   "NeedsLongHelp");
        assert_eq!($flags1.is_set(AppSettings::NeedsLongVersion),
                   $flags2.is_set(AppSettings::NeedsLongVersion),
                   "NeedsLongVersion");
        assert_eq!($flags1.is_set(AppSettings::NeedsSubcommandHelp),
                   $flags2.is_set(AppSettings::NeedsSubcommandHelp),
                   "NeedsSubcommandHelp");
        assert_eq!($flags1.is_set(AppSettings::NoBinaryName),
                   $flags2.is_set(AppSettings::NoBinaryName),
                   "NoBinaryName");
        assert_eq!($flags1.is_set(AppSettings::StrictUtf8),
                   $flags2.is_set(AppSettings::StrictUtf8),
                   "StrictUtf8");
        assert_eq!($flags1.is_set(AppSettings::SubcommandsNegateReqs),
                   $flags2.is_set(AppSettings::SubcommandsNegateReqs),
                   "SubcommandsNegateReqs");
        assert_eq!($flags1.is_set(AppSettings::SubcommandRequired),
                   $flags2.is_set(AppSettings::SubcommandRequired),
                   "SubcommandRequired");
        assert_eq!($flags1.is_set(AppSettings::SubcommandRequiredElseHelp),
                   $flags2.is_set(AppSettings::SubcommandRequiredElseHelp),
                   "SubcommandRequiredElseHelp");
        assert_eq!($flags1.is_set(AppSettings::TrailingVarArg),
                   $flags2.is_set(AppSettings::TrailingVarArg),
                   "TrailingVarArg");
        assert_eq!($flags1.is_set(AppSettings::UnifiedHelpMessage),
                   $flags2.is_set(AppSettings::UnifiedHelpMessage),
                   "UnifiedHelpMessage");
        assert_eq!($flags1.is_set(AppSettings::NextLineHelp),
                   $flags2.is_set(AppSettings::NextLineHelp),
                   "NextLineHelp");
        assert_eq!($flags1.is_set(AppSettings::VersionlessSubcommands),
                   $flags2.is_set(AppSettings::VersionlessSubcommands),
                   "VersionlessSubcommands");
        assert_eq!($flags1.is_set(AppSettings::WaitOnError),
                   $flags2.is_set(AppSettings::WaitOnError),
                   "WaitOnError");
        assert_eq!($flags1.is_set(AppSettings::TrailingValues),
                   $flags2.is_set(AppSettings::TrailingValues),
                   "TrailingValues");
        assert_eq!($flags1.is_set(AppSettings::ValidNegNumFound),
                   $flags2.is_set(AppSettings::ValidNegNumFound),
                   "ValidNegNumFound");
        assert_eq!($flags1.is_set(AppSettings::Propagated),
                   $flags2.is_set(AppSettings::Propagated),
                   "Propagated");
        assert_eq!($flags1.is_set(AppSettings::ValidArgFound),
                   $flags2.is_set(AppSettings::ValidArgFound),
                   "ValidArgFound");
        assert_eq!($flags1.is_set(AppSettings::InferSubcommands),
                   $flags2.is_set(AppSettings::InferSubcommands),
                   "InferSubcommands");
        assert_eq!($flags1.is_set(AppSettings::ContainsLast),
                   $flags2.is_set(AppSettings::ContainsLast),
                   "ContainsLast");
    };
}

macro_rules! compare_base {
    ($base1:expr, $base2:expr) => {
        assert_eq!($base1.name, $base2.name, "name");
        assert_eq!($base1.help, $base2.help, "help");
        // Dont test the long help while we are transitioning between clap_app and structopt
        // assert_eq!($base1.long_help, $base2.long_help, "long_help");
        assert_eq!($base1.blacklist, $base2.blacklist, "blacklist");
        assert_eq!($base1.is_set(ArgSettings::Required),
                   $base2.is_set(ArgSettings::Required),
                   "Required");
        assert_eq!($base1.is_set(ArgSettings::Multiple),
                   $base2.is_set(ArgSettings::Multiple),
                   "Multiple");
        assert_eq!($base1.is_set(ArgSettings::EmptyValues),
                   $base2.is_set(ArgSettings::EmptyValues),
                   "EmptyValues");
        assert_eq!($base1.is_set(ArgSettings::Global),
                   $base2.is_set(ArgSettings::Global),
                   "Global");
        assert_eq!($base1.is_set(ArgSettings::Hidden),
                   $base2.is_set(ArgSettings::Hidden),
                   "Hidden");
        assert_eq!($base1.is_set(ArgSettings::TakesValue),
                   $base2.is_set(ArgSettings::TakesValue),
                   "TakesValue");
        assert_eq!($base1.is_set(ArgSettings::UseValueDelimiter),
                   $base2.is_set(ArgSettings::UseValueDelimiter),
                   "UseValueDelimiter");
        assert_eq!($base1.is_set(ArgSettings::NextLineHelp),
                   $base2.is_set(ArgSettings::NextLineHelp),
                   "NextLineHelp");
        assert_eq!($base1.is_set(ArgSettings::RequireDelimiter),
                   $base2.is_set(ArgSettings::RequireDelimiter),
                   "RequireDelimiter");
        assert_eq!($base1.is_set(ArgSettings::HidePossibleValues),
                   $base2.is_set(ArgSettings::HidePossibleValues),
                   "HidePossibleValues");
        assert_eq!($base1.is_set(ArgSettings::AllowLeadingHyphen),
                   $base2.is_set(ArgSettings::AllowLeadingHyphen),
                   "AllowLeadingHyphen");
        assert_eq!($base1.is_set(ArgSettings::RequireEquals),
                   $base2.is_set(ArgSettings::RequireEquals),
                   "RequireEquals");
        assert_eq!($base1.is_set(ArgSettings::Last),
                   $base2.is_set(ArgSettings::Last),
                   "Last");
        assert_eq!($base1.is_set(ArgSettings::HideDefaultValue),
                   $base2.is_set(ArgSettings::HideDefaultValue),
                   "HideDefaultValue");
        assert_eq!($base1.is_set(ArgSettings::CaseInsensitive),
                   $base2.is_set(ArgSettings::CaseInsensitive),
                   "CaseInsensitive");
        assert_eq!($base1.is_set(ArgSettings::HideEnvValues),
                   $base2.is_set(ArgSettings::HideEnvValues),
                   "HideEnvValues");
        assert_eq!($base1.is_set(ArgSettings::HiddenShortHelp),
                   $base2.is_set(ArgSettings::HiddenShortHelp),
                   "HiddenShortHelp");
        assert_eq!($base1.is_set(ArgSettings::HiddenLongHelp),
                   $base2.is_set(ArgSettings::HiddenLongHelp),
                   "HiddenLongHelp");
        assert_eq!($base1.is_set(ArgSettings::RequiredUnlessAll),
                   $base2.is_set(ArgSettings::RequiredUnlessAll),
                   "RequiredUnlessAll");
        assert_eq!($base1.is_set(ArgSettings::ValueDelimiterNotSet),
                   $base2.is_set(ArgSettings::ValueDelimiterNotSet),
                   "ValueDelimiterNotSet");
        assert_eq!($base1.r_unless, $base2.r_unless, "r_unless");
        assert_eq!($base1.overrides, $base2.overrides, "overrides");
        // The clap_app macro and structopt treat groups slightly differently making this
        // impractical to check
        // assert_eq!($base1.groups, $base2.groups, "groups");
        assert_eq!($base1.requires, $base2.requires, "requires");
    };
}

macro_rules! compare_switched {
    ($switched1:expr, $switched2:expr) => {
        assert_eq!($switched1.short, $switched2.short, "short");
        assert_eq!($switched1.long, $switched2.long, "long");
        assert_eq!($switched1.aliases, $switched2.aliases, "aliases");
        assert_eq!($switched1.disp_ord, $switched2.disp_ord, "disp_ord");
        // No need to check the unified order
        // assert_eq!($switched1.unified_ord, $switched2.unified_ord,
        //            "unified_ord");
    };
}

macro_rules! compare_valued {
    ($valued1:expr, $valued2:expr) => {
        assert_eq!($valued1.possible_vals, $valued2.possible_vals,
                   "possible_vals");
        assert_eq!($valued1.val_names, $valued2.val_names, "val_names");
        assert_eq!($valued1.num_vals, $valued2.num_vals, "num_vals");
        assert_eq!($valued1.max_vals, $valued2.max_vals, "max_vals");
        assert_eq!($valued1.min_vals, $valued2.min_vals, "min_vals");
        assert_eq!($valued1.val_delim, $valued2.val_delim, "val_delim");
        assert_eq!($valued1.default_val, $valued2.default_val, "default_val");
        assert_eq!($valued1.default_vals_ifs, $valued2.default_vals_ifs,
                   "default_vals_ifs");
        assert_eq!($valued1.env, $valued2.env, "env");
        assert_eq!($valued1.terminator, $valued2.terminator, "terminator");
    };
}

#[allow(clippy::cognitive_complexity)]
fn compare(app1: &mut App, app2: &mut App, path: &str) {
    println!("=== Comparing app '{}' subcommands '{} and '{}' ===",
             path, app1.p.meta.name, app2.p.meta.name);

    let p1 = &mut app1.p;
    let p2 = &mut app2.p;

    // Ignore config file flags
    p1.opts.retain(|f| f.b.name != "generate-config");
    p2.opts.retain(|f| f.b.name != "generate-config");
    p1.opts.retain(|f| f.b.name != "config-files");
    p2.opts.retain(|f| f.b.name != "config-files");
    p1.opts.retain(|f| f.b.name != "GENERATE_CONFIG");
    p2.opts.retain(|f| f.b.name != "GENERATE_CONFIG");
    p1.opts.retain(|f| f.b.name != "CONFIG_FILES");
    p2.opts.retain(|f| f.b.name != "CONFIG_FILES");

    // Compare help messages
    let help1 = help(app1);
    let help2 = help(app2);
    if help1 != help2 {
        println!("{}", help1);
        println!("================================");
        println!("{}", help2);
        assert_eq!(help1, help2);
    }

    let p1 = &mut app1.p;
    let p2 = &mut app2.p;

    println!("Comparing app meta");
    assert_eq!(p1.meta.name, p2.meta.name, "meta name");
    assert_eq!(p1.meta.bin_name, p2.meta.bin_name, "meta bin_name");
    assert_eq!(p1.meta.author, p2.meta.author, "meta author");
    assert_eq!(p1.meta.version, p2.meta.version, "meta version");
    assert_eq!(p1.meta.long_version, p2.meta.long_version,
               "meta long_version");
    assert_eq!(p1.meta.about, p2.meta.about, "meta about");
    assert_eq!(p1.meta.long_about, p2.meta.long_about, "meta long_about");
    assert_eq!(p1.meta.more_help, p2.meta.more_help, "meta more_help");
    assert_eq!(p1.meta.pre_help, p2.meta.pre_help, "meta pre_help");
    // We intentially do not compare aliases
    // assert_eq!(p1.meta.aliases, p2.meta.aliases, "meta aliases");
    assert_eq!(p1.meta.usage_str, p2.meta.usage_str, "meta usage_str");
    assert_eq!(p1.meta.usage, p2.meta.usage, "meta usage");
    assert_eq!(p1.meta.help_str, p2.meta.help_str, "meta help_str");
    assert_eq!(p1.meta.disp_ord, p2.meta.disp_ord, "meta disp_ord");
    assert_eq!(p1.meta.term_w, p2.meta.term_w, "meta term_w");
    assert_eq!(p1.meta.max_w, p2.meta.max_w, "meta max_w");
    assert_eq!(p1.meta.template, p2.meta.template, "meta template");

    println!("Comparing app flags");
    compare_app_flags!(p1, p2);

    println!("Comparing app global flags");
    compare_app_flags!(p1.g_settings, p2.g_settings);

    // Check flags
    p1.flags.sort_by_key(|f| f.b.name);
    p2.flags.sort_by_key(|f| f.b.name);
    for (flag1, flag2) in p1.flags.iter().zip(p2.flags.iter()) {
        println!("Comparing flag '{}' and '{}'", flag1.b.name, flag2.b.name,);
        compare_base!(flag1.b, flag2.b);
        compare_switched!(flag1.s, flag2.s);
    }
    assert_eq!(p1.flags.len(), p2.flags.len(), "flags length");

    // Check options
    p1.opts.sort_by_key(|o| o.b.name);
    p2.opts.sort_by_key(|o| o.b.name);
    for (opt1, opt2) in p1.opts.iter().zip(p2.opts.iter()) {
        println!("Comparing opt '{}' and '{}'", opt1.b.name, opt2.b.name,);
        compare_base!(opt1.b, opt2.b);
        compare_switched!(opt1.s, opt2.s);
        compare_valued!(opt1.v, opt2.v);
    }
    assert_eq!(p1.opts.len(), p2.opts.len(), "opts length");

    // Check positionals
    for ((index1, positional1), (index2, positional2)) in
        p1.positionals.iter().zip(p2.positionals.iter())
    {
        println!("Comparing positional {} to {}",
                 positional1.b.name, positional2.b.name,);
        compare_base!(positional1.b, positional2.b);
        compare_valued!(positional1.v, positional2.v);
        assert_eq!(index1, index2, "positional index");
        assert_eq!(positional1.index, positional2.index, "positional index2");
    }
    assert_eq!(p1.positionals.len(),
               p2.positionals.len(),
               "positionals length");

    // Check subcommands
    p1.subcommands.sort_by_key(|s| s.p.meta.name.clone());
    p2.subcommands.sort_by_key(|s| s.p.meta.name.clone());
    for (sub1, sub2) in p1.subcommands.iter_mut().zip(p2.subcommands.iter_mut()) {
        let path = format!("{}::{}", path, sub1.p.meta.name);
        compare(sub1, sub2, &path);
    }
    assert_eq!(p1.subcommands.len(),
               p2.subcommands.len(),
               "subcommands length");

    // Check groups
    p1.groups.sort_by_key(|g| g.name);
    p2.groups.sort_by_key(|g| g.name);
    for (group1, group2) in p1.groups.iter_mut().zip(p2.groups.iter_mut()) {
        println!("Comparing group {} to {}", group1.name, group2.name,);
        assert_eq!(group1.name, group2.name, "groups name");
        assert_eq!(group1.args, group2.args, "groups args");
        assert_eq!(group1.required, group2.required, "groups required");
        assert_eq!(group1.requires, group2.requires, "groups requires");
        assert_eq!(group1.conflicts, group2.conflicts, "groups conflicts");
        assert_eq!(group1.multiple, group2.multiple, "groups multiple");
    }
    assert_eq!(p1.groups.len(), p2.groups.len(), "groups length");

    // Check global args
    p1.global_args.sort_by_key(|g| g.b.name);
    p2.global_args.sort_by_key(|g| g.b.name);
    for (global_arg1, global_arg2) in p1.global_args.iter_mut().zip(p2.global_args.iter_mut()) {
        println!("Comparing global_arg {} to {}",
                 global_arg1.b.name, global_arg2.b.name,);
        compare_base!(global_arg1.b, global_arg2.b);
        compare_switched!(global_arg1.s, global_arg2.s);
        compare_valued!(global_arg1.v, global_arg2.v);
        assert_eq!(global_arg1.index, global_arg2.index, "global_arg index");
        assert_eq!(global_arg1.r_ifs, global_arg2.r_ifs, "global_arg r_ifs");
    }
    assert_eq!(p1.global_args.len(),
               p2.global_args.len(),
               "global_args length");

    assert_eq!(p1.required, p2.required, "parser required");
    assert_eq!(p1.r_ifs, p2.r_ifs, "parser r_ifs");
    assert_eq!(p1.overrides, p2.overrides, "parser overrides");
    assert_eq!(p1.help_message, p2.help_message, "parser help_message");
    assert_eq!(p1.version_message, p2.version_message,
               "parser version_message");
}

#[test]
fn test_hab_help() {
    habitat_core::locked_env_var!(HAB_FEAT_SERVICE_CONFIG_FILES, lock_service_config_files);
    let env = lock_service_config_files();
    env.set("1");

    let mut hab1 = cli::get(feature_flags_for_cli_test());
    let mut hab2 = cli::get(feature_flags_for_cli_test_with_structopt());
    compare(&mut hab1, &mut hab2, "hab");
}

fn extract_hab_svc_load(hab: Hab) -> Load {
    if let Hab::Svc(Svc::Load(load)) = hab {
        load
    } else {
        panic!("expected to find `hab svc load`")
    }
}

fn extract_hab_svc_update(hab: Hab) -> Update {
    if let Hab::Svc(Svc::Update(update)) = hab {
        update
    } else {
        panic!("expected to find `hab svc update`")
    }
}

#[test]
fn test_hab_svc_load_flag_ordering() {
    let pkg_ident = "core/redis".parse().unwrap();

    let hab = Hab::try_from_iter_with_configopt(&["hab", "svc", "load", "core/redis"]).unwrap();
    let load = extract_hab_svc_load(hab);
    assert!(!load.force);
    assert_eq!(load.pkg_ident.pkg_ident(), pkg_ident);

    let hab = Hab::try_from_iter_with_configopt(&["hab", "svc", "load", "--force", "core/redis"]).unwrap();
    let load = extract_hab_svc_load(hab);
    assert!(load.force);
    assert_eq!(load.pkg_ident.pkg_ident(), pkg_ident);

    let hab = Hab::try_from_iter_with_configopt(&["hab", "svc", "load", "core/redis", "--force"]).unwrap();
    let load = extract_hab_svc_load(hab);
    assert!(load.force);
    assert_eq!(load.pkg_ident.pkg_ident(), pkg_ident);
}

#[test]
fn test_hab_svc_update_empty_binds() {
    let hab = Hab::try_from_iter_with_configopt(&["hab", "svc", "update", "core/redis", "--bind"]).unwrap();
    let update = extract_hab_svc_update(hab);
    assert_eq!(update.bind, Some(vec![]));

    let hab = Hab::try_from_iter_with_configopt(&["hab",
                                                  "svc",
                                                  "update",
                                                  "core/redis",
                                                  "--bind",
                                                  "x:y.z"]).unwrap();
    let update = extract_hab_svc_update(hab);
    assert_eq!(update.bind.unwrap().len(), 1);
}
