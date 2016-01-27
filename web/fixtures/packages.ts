export default [
  {
    name: "glibc",
    derivation: "chef",
    version: "2.19",
    release: "20160111220307",
    buildDependencies: [],
    dependencies: [],
  },
  {
    name: "zlib",
    derivation: "chef",
    version: "1.2.8",
    release: "20160111220313",
    buildDependencies: [],
    dependencies: [],
  },
  {
    name: "cacerts",
    derivation: "chef",
    version: "2016.01.11",
    release: "20160111220317",
    buildDependencies: [],
    dependencies: [],
  },
  {
    name: "openssl",
    derivation: "smith",
    version: "1.0.2d",
    release: "20160101000000",
    license: "BSD",
    maintainer: "Jamie Winsor <reset@chef.io>",
    source: "https://www.openssl.org/source/openssl-1.0.2d.tar.gz",
    sha: "e23ccafdb75cfcde782da0151731aa2185195ac745eea3846133f2e05c0e0bff",
    buildDependencies: [],
    dependencies: [
      {
        name: "glibc",
        derivation: "chef",
        version: "2.19",
        release: "20160111220307",
      },
      {
        name: "zlib",
        derivation: "chef",
        version: "1.2.8",
        release: "20160111220313",
      },
      {
        name: "cacerts",
        derivation: "chef",
        version: "2016.01.11",
        release: "20160111220317",
      }
    ]
  },
  {
    name: "openssl",
    derivation: "smith",
    version: "1.0.2e",
    release: "20160111220549",
    maintainer: "Jamie Winsor <reset@chef.io>",
    license: "BSD",
    source: "https://www.openssl.org/source/openssl-1.0.2e.tar.gz",
    sha: "e23ccafdb75cfcde782da0151731aa2185195ac745eea3846133f2e05c0e0bff",
    buildDependencies: [],
    dependencies: [
      {
        name: "glibc",
        derivation: "chef",
        version: "2.19",
        release: "20160111220307",
      },
      {
        name: "zlib",
        derivation: "chef",
        version: "1.2.8",
        release: "20160111220313",
      },
      {
        name: "cacerts",
        derivation: "chef",
        version: "2016.01.11",
        release: "20160111220317",
      }
    ]
  },
  {
    name: "openssl",
    derivation: "smith",
    version: "1.0.2e",
    release: "20160102000000",
    maintainer: "Jamie Winsor <reset@chef.io>",
    license: "BSD",
    source: "https://www.openssl.org/source/openssl-1.0.2e.tar.gz",
    sha: "e23ccafdb75cfcde782da0151731aa2185195ac745eea3846133f2e05c0e0bff",
    buildDependencies: [],
    dependencies: [
      {
        name: "glibc",
        derivation: "chef",
        version: "2.19",
        release: "20160111220307",
      },
      {
        name: "zlib",
        derivation: "chef",
        version: "1.2.8",
        release: "20160111220313",
      },
      {
        name: "cacerts",
        derivation: "chef",
        version: "2016.01.11",
        release: "20160111220317",
      }
    ]
  },
  {
    name: "openssl",
    derivation: "smith",
    version: "1.0.2e",
    release: "20160101000000",
    license: "BSD",
    maintainer: "Jamie Winsor <reset@chef.io>",
    source: "https://www.openssl.org/source/openssl-1.0.2e.tar.gz",
    sha: "e23ccafdb75cfcde782da0151731aa2185195ac745eea3846133f2e05c0e0bff",
    buildDependencies: [],
    dependencies: [
      {
        name: "glibc",
        derivation: "chef",
        version: "2.19",
        release: "20160111220307",
      },
      {
        name: "zlib",
        derivation: "chef",
        version: "1.2.8",
        release: "20160111220313",
      },
      {
        name: "cacerts",
        derivation: "chef",
        version: "2016.01.11",
        release: "20160111220317",
      }
    ]
  },
  {
    name: "openssl",
    derivation: "smith",
    version: "1.0.2d",
    release: "20160102000000",
    license: "BSD",
    maintainer: "Jamie Winsor <reset@chef.io>",
    source: "https://www.openssl.org/source/openssl-1.0.2e.tar.gz",
    sha: "e23ccafdb75cfcde782da0151731aa2185195ac745eea3846133f2e05c0e0bff",
    buildDependencies: [],
    dependencies: [
      {
        name: "glibc",
        derivation: "chef",
        version: "2.19",
        release: "20160111220307",
      },
      {
        name: "zlib",
        derivation: "chef",
        version: "1.2.8",
        release: "20160111220313",
      },
      {
        name: "cacerts",
        derivation: "chef",
        version: "2016.01.11",
        release: "20160111220317",
      }
    ]
  },
  {
    name: "runit",
    derivation: "smith",
    version: "2.1.2",
    release: "20160111220840",
    description: "It cannot be stopped.",
    dependencies: [
      {
        name: "glibc",
        derivation: "chef",
        version: "2.19",
        release: "20160111220307",
      }
    ],
    maintainer: "Joshua Timberman <jtimberman@chef.io>",
    license: "BSD",
    source: "http://smarden.org/runit/runit-2.1.2.tar.gz",
    sha: "6fd0160cb0cf1207de4e66754b6d39750cff14bb0aa66ab49490992c0c47ba18",
    buildDependencies: [],
  },
];
