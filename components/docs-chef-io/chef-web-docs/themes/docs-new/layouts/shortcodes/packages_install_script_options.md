In addition to the default install behavior, the Chef Software Install script
supports the following options:

`-c` (`-channel` on Microsoft Windows)

:   The release channel from which a package is pulled. Possible values:
    `current` or `stable`. Default value: `stable`.

`-d` (`-download_directory` on Microsoft Windows)

:   The directory into which a package is downloaded. When a package
    already exists in this directory and the checksum matches, the
    package is not re-downloaded. When `-d` and `-f` are not specified,
    a package is downloaded to a temporary directory.

`-f` (`-filename` on Microsoft Windows)

:   The name of the file and the path at which that file is located.
    When a filename already exists at this path and the checksum
    matches, the package is not re-downloaded. When `-d` and `-f` are
    not specified, a package is downloaded to a temporary directory.

`-P` (`-project` on Microsoft Windows)

:   The product name to install. Supported versions of Chef products are
    `automate`, `chef`, `chef-server`, `inspec`, `chef-workstation`,
    `chefdk`, `supermarket`, `chef-backend`, `push-jobs-client`, and
    `push-jobs-server`. Default value: `chef`.

`-s` (`-install_strategy` on Microsoft Windows)

:   The method of package installations. The default strategy is to
    always install when the install.sh script runs. Set to "once" to
    skip installation if the product is already installed on the node.

`-l` (`-download_url_override` on Microsoft Windows)

:   Install package downloaded from a direct URL.

`-a` (`-checksum` on Microsoft Windows)

:   The SHA256 for download_url_override

`-v` (`-version` on Microsoft Windows)

:   The version of the package to be installed. A version always takes
    the form x.y.z, where x, y, and z are decimal numbers that are used
    to represent major (x), minor (y), and patch (z) versions. A
    two-part version (x.y) is also allowed. For more information about
    application versioning, see <https://semver.org/>.