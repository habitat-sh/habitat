use crate::{error::Result,
            RegistryType};
use failure::SyncFailure;
use habitat_core::{package::{FullyQualifiedPackageIdent,
                             Identifiable},
                   ChannelIdent};
use handlebars::Handlebars;

/// An image naming policy.
///
/// This is a value struct which captures the naming and tagging intentions for an image.
#[derive(Debug, Default)]
pub struct Naming {
    /// An optional custom image name which would override a computed default value.
    custom_image_name:   Option<String>,
    /// Whether or not to tag the image with a latest value.
    latest_tag:          bool,
    /// Whether or not to tag the image with a value containing a version from a Package
    /// Identifier.
    version_tag:         bool,
    /// Whether or not to tag the image with a value containing a version and release from a
    /// Package Identifier.
    version_release_tag: bool,
    /// An optional custom tag value for the image.
    custom_tag:          Option<String>,

    // TODO (CM): I don't think either of these really belongs to this
    // Naming type

    // TODO (CM): really, url and type are conceptually part of a
    // single new type.
    /// A URL to a custom Docker registry to publish to. This will be used as part of every tag
    /// before pushing.
    pub registry_url:  Option<String>, // TODO (CM): Option<Url>
    /// The type of registry we're publishing to. Ex: Amazon, Docker, Google, Azure.
    pub registry_type: RegistryType,
}

// TODO (CM): must validate custom names and tags w/r/t tag spec
// https://docs.docker.com/engine/reference/commandline/tag/
//
// An image name is made up of slash-separated name components,
// optionally prefixed by a registry hostname. The hostname must
// comply with standard DNS rules, but may not contain underscores. If
// a hostname is present, it may optionally be followed by a port
// number in the format :8080. If not present, the command uses
// Docker’s public registry located at registry-1.docker.io by
// default. Name components may contain lowercase letters, digits and
// separators. A separator is defined as a period, one or two
// underscores, or one or more dashes. A name component may not start
// or end with a separator.
//
// A tag name must be valid ASCII and may contain lowercase and
// uppercase letters, digits, underscores, periods and dashes. A tag
// name may not start with a period or a dash and may contain a
// maximum of 128 characters.

impl Naming {
    /// Creates a `Naming` from cli arguments.
    pub fn new_from_cli_matches(m: &clap::ArgMatches) -> Self {
        let registry_type =
            clap::value_t!(m.value_of("REGISTRY_TYPE"), RegistryType).unwrap_or_default();

        // TODO (CM): If registry_type is Docker, we must set this to
        // dockerhub. Otherwise, it MUST be present, because of how
        // clap is set up.
        let registry_url = m.value_of("REGISTRY_URL").map(ToString::to_string);

        Naming { custom_image_name: m.value_of("IMAGE_NAME").map(ToString::to_string),
                 latest_tag: !m.is_present("NO_TAG_LATEST"),
                 version_tag: !m.is_present("NO_TAG_VERSION"),
                 version_release_tag: !m.is_present("NO_TAG_VERSION_RELEASE"),
                 custom_tag: m.value_of("TAG_CUSTOM").map(ToString::to_string),
                 registry_url,
                 registry_type }
    }

    pub fn image_identifiers(&self,
                             ident: &FullyQualifiedPackageIdent,
                             channel: &ChannelIdent)
                             -> Result<(String, Vec<String>)> {
        let name = self.image_name(ident, channel)?;
        let tags = vec![self.latest_tag(),
                        self.version_tag(&ident),
                        self.version_release_tag(&ident),
                        self.custom_tag(ident, channel)?].into_iter()
                                                         .filter_map(|e| e)
                                                         .collect();
        Ok((name, tags))
    }

    fn image_name(&self,
                  ident: &FullyQualifiedPackageIdent,
                  channel: &ChannelIdent)
                  -> Result<String> {
        let json = Self::json_payload(ident, channel);

        let image_name = match self.custom_image_name {
            Some(ref custom) => {
                Handlebars::new().template_render(custom, &json)
                                 .map_err(SyncFailure::new)?
            }
            None => format!("{}/{}", ident.origin(), ident.name()),
        };

        // TODO (CM): perhaps we should prepend the registry URL to
        // tags only if we going to push (and at the time we are
        // pushing, not here)
        let image_name = match self.registry_url {
            Some(ref url) => format!("{}/{}", url, image_name),
            None => image_name,
        };

        Ok(image_name.to_lowercase())
    }

    fn version_release_tag(&self, ident: &FullyQualifiedPackageIdent) -> Option<String> {
        if self.version_release_tag {
            Some(format!("{}-{}",
                         FullyQualifiedPackageIdent::version(ident),
                         FullyQualifiedPackageIdent::release(ident)))
        } else {
            None
        }
    }

    fn version_tag(&self, ident: &FullyQualifiedPackageIdent) -> Option<String> {
        if self.version_tag {
            Some(FullyQualifiedPackageIdent::version(ident).to_string())
        } else {
            None
        }
    }

    // TODO (CM): not sure how useful this is, since I think "latest"
    // is *always* created.
    fn latest_tag(&self) -> Option<String> {
        if self.latest_tag {
            Some("latest".to_string())
        } else {
            None
        }
    }

    fn custom_tag(&self,
                  ident: &FullyQualifiedPackageIdent,
                  channel: &ChannelIdent)
                  -> Result<Option<String>> {
        if let Some(ref custom) = self.custom_tag {
            let json = Self::json_payload(ident, channel);
            let tag = Handlebars::new().template_render(&custom, &json)
                                       .map_err(SyncFailure::new)
                                       .map(|s| s.to_lowercase())?;
            Ok(Some(tag))
        } else {
            Ok(None)
        }
    }

    fn json_payload(ident: &FullyQualifiedPackageIdent,
                    channel: &ChannelIdent)
                    -> serde_json::Value {
        json!({
            "pkg_origin": ident.origin(),
            "pkg_name": ident.name(),
            "pkg_version": ident.version(),
            "pkg_release": ident.release(),
            "channel": channel,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ident() -> FullyQualifiedPackageIdent { "core/foo/1.2.3/20200430153200".parse().unwrap() }

    #[test]
    fn default_naming_policy() {
        let naming = Naming::default();
        let ident = ident();
        let channel = ChannelIdent::default();

        assert!(naming.latest_tag().is_none());
        assert!(naming.custom_tag(&ident, &channel).unwrap().is_none());
        assert!(naming.version_tag(&ident).is_none());
        assert!(naming.version_release_tag(&ident).is_none());

        assert_eq!(naming.image_name(&ident, &channel).unwrap(), "core/foo");
    }

    #[test]
    fn latest_tag() {
        let mut naming = Naming::default();
        naming.latest_tag = true;
        assert_eq!(naming.latest_tag().unwrap(), "latest");
    }

    #[test]
    fn version_tag() {
        let mut naming = Naming::default();
        naming.version_tag = true;
        assert_eq!(naming.version_tag(&ident()).unwrap(), "1.2.3");
    }

    #[test]
    fn version_release_tag() {
        let mut naming = Naming::default();
        naming.version_release_tag = true;
        assert_eq!(naming.version_release_tag(&ident()).unwrap(),
                   "1.2.3-20200430153200");
    }

    #[test]
    fn image_name_with_registry_url() {
        let ident = ident();
        let channel = ChannelIdent::default();

        let mut naming = Naming::default();
        // TODO (CM): IMPLEMENTATION QUIRK
        // Registry type has no bearing on this! Fix it!
        naming.registry_url = Some(String::from("registry.mycompany.com:8080/v1"));

        let name = naming.image_name(&ident, &channel).unwrap();
        assert_eq!(name, "registry.mycompany.com:8080/v1/core/foo");
    }

    #[test]
    fn custom_image_names() {
        let ident = ident();
        let channel = ChannelIdent::default();

        // Template, Expected Result
        //
        // A "Some" expected result is something you expect to successfully
        // pass. A "None", on the other hand, is something you expect
        // to throw an error.
        let inputs = vec![

            // Valid inputs
            ("monkeys", Some("monkeys")),
            ("{{pkg_name}}", Some("foo")),
            ("{{pkg_origin}}-{{pkg_name}}", Some("core-foo")),
            ("{{pkg_origin}}-{{pkg_name}}-{{pkg_version}}", Some("core-foo-1.2.3")),
            ("{{pkg_origin}}-{{pkg_name}}-{{pkg_version}}-{{pkg_release}}",
             Some("core-foo-1.2.3-20200430153200")),
            ("{{pkg_origin}}-{{pkg_name}}-{{pkg_version}}-{{pkg_release}}-{{channel}}",
             Some("core-foo-1.2.3-20200430153200-stable")),
            ("super-{{pkg_origin}}-wacky-{{pkg_name}}-funtime-{{pkg_version}}-container-{{pkg_release}}-party-{{channel}}-ohemgee",
             Some("super-core-wacky-foo-funtime-1.2.3-container-20200430153200-party-stable-ohemgee")),

            // Invalid inputs
            ("{{" , None),
            // ("{{not_a_valid_variable}}-{{pkg_name}}", None),
            // ("", None),
            // more examples of things that violate the tagging spec
        ];

        for (template, expected) in inputs {
            let mut naming = Naming::default();

            let template = String::from(template);
            naming.custom_image_name = Some(template.clone());
            let actual_name = naming.image_name(&ident, &channel);

            if let Some(expected_name) = expected {
                // expected success
                assert!(actual_name.is_ok());
                let actual_name = actual_name.unwrap();
                assert_eq!(actual_name, expected_name,
                           "Expected template '{}' to generate image name '{}', but it generated \
                            '{}'",
                           template, expected_name, actual_name);
            } else {
                // expected failure
                assert!(actual_name.is_err(),
                        "Expected template '{}' to fail to generate an image name, but it \
                         generated '{}'",
                        template,
                        actual_name.unwrap());
            }
        }
    }

    // TODO (CM): there is a bunch of duplication here :(

    #[test]
    fn custom_tag_names() {
        let ident = ident();
        let channel = ChannelIdent::default();

        // Template, Expected Result
        //
        // A "Some" expected result is something you expect to successfully
        // pass. A "None", on the other hand, is something you expect
        // to throw an error.
        let inputs = vec![

            // Valid inputs
            ("monkeys", Some("monkeys")),
            ("{{pkg_name}}", Some("foo")),
            ("{{pkg_origin}}-{{pkg_name}}", Some("core-foo")),
            ("{{pkg_origin}}-{{pkg_name}}-{{pkg_version}}", Some("core-foo-1.2.3")),
            ("{{pkg_origin}}-{{pkg_name}}-{{pkg_version}}-{{pkg_release}}",
             Some("core-foo-1.2.3-20200430153200")),
            ("{{pkg_origin}}-{{pkg_name}}-{{pkg_version}}-{{pkg_release}}-{{channel}}",
             Some("core-foo-1.2.3-20200430153200-stable")),
            ("super-{{pkg_origin}}-wacky-{{pkg_name}}-funtime-{{pkg_version}}-container-{{pkg_release}}-party-{{channel}}-ohemgee",
             Some("super-core-wacky-foo-funtime-1.2.3-container-20200430153200-party-stable-ohemgee")),

            // Invalid inputs
            ("{{" , None),
            // ("{{not_a_valid_variable}}-{{pkg_name}}", None),
            // ("", None),
            // more examples of things that violate the tagging spec
        ];

        for (template, expected) in inputs {
            let mut naming = Naming::default();

            let template = String::from(template);
            naming.custom_tag = Some(template.clone());
            let actual_tag = naming.custom_tag(&ident, &channel);

            if let Some(expected_tag) = expected {
                // expected success
                assert!(actual_tag.is_ok());
                let actual_tag = actual_tag.unwrap().unwrap(); // Result<Option<String>>
                assert_eq!(actual_tag, expected_tag,
                           "Expected template '{}' to generate tag '{}', but it generated '{}'",
                           template, expected_tag, actual_tag);
            } else {
                // expected failure
                assert!(actual_tag.is_err(),
                        "Expected template '{}' to fail to generate a tag, but it generated '{}'",
                        template,
                        actual_tag.unwrap().unwrap());
            }
        }
    }

    #[test]
    fn default_image_identifiers() {
        let naming = Naming::default();
        let ident = ident();
        let channel = ChannelIdent::default();

        let (name, tags) = naming.image_identifiers(&ident, &channel).unwrap();

        assert_eq!(name, "core/foo");
        assert!(tags.is_empty());
    }

    #[test]
    fn all_the_image_identifiers() {
        let naming = Naming { custom_image_name:   Some(String::from("my-nifty/{{pkg_name}}")),
                              latest_tag:          true,
                              version_tag:         true,
                              version_release_tag: true,
                              custom_tag:          Some(String::from("new-hotness")),
                              registry_url:        Some(String::from("registry.mycompany.com:\
                                                                      8080/v1")),
                              registry_type:       RegistryType::Docker, };

        let ident = ident();
        let channel = ChannelIdent::default();

        let (name, tags) = naming.image_identifiers(&ident, &channel).unwrap();

        assert_eq!(name, "registry.mycompany.com:8080/v1/my-nifty/foo");
        assert_eq!(tags,
                   ["latest", "1.2.3", "1.2.3-20200430153200", "new-hotness"]);
    }
}
