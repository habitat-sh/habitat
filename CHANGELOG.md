# Habitat CHANGELOG

## [0.12.0](https://github.com/habitat-sh/habitat/tree/0.12.0) (10-28-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.11.0...0.12.0)

## Features & Enhancements

- Docs: Link to missing wikipedia page [\#1352](https://github.com/habitat-sh/habitat/issues/1352)
- Docs: Link to missing wikipedia page [\#1352](https://github.com/habitat-sh/habitat/issues/1352)
- habitat shouldn't require sudo [\#903](https://github.com/habitat-sh/habitat/issues/903)
- Now that swim is butterfly, fix the Makefile to reflect that [\#1391](https://github.com/habitat-sh/habitat/pull/1391) ([smurawski](https://github.com/smurawski))
- add a `hab pkg header` command to read the hart file header [\#1388](https://github.com/habitat-sh/habitat/pull/1388) ([smurawski](https://github.com/smurawski))
- \[core\] Fix hyper test compilation churn. [\#1383](https://github.com/habitat-sh/habitat/pull/1383) ([fnichol](https://github.com/fnichol))
- Added myself as a maintainer [\#1378](https://github.com/habitat-sh/habitat/pull/1378) ([eeyun](https://github.com/eeyun))
- Fix broken Wikipedia link [\#1375](https://github.com/habitat-sh/habitat/pull/1375) ([juliandunn](https://github.com/juliandunn))
- Minor Admin API doc corrections [\#1373](https://github.com/habitat-sh/habitat/pull/1373) ([reset](https://github.com/reset))
- Display private key names on the origin page. [\#1371](https://github.com/habitat-sh/habitat/pull/1371) ([raskchanky](https://github.com/raskchanky))
- Appveyor building `hab` and testing ported crates [\#1370](https://github.com/habitat-sh/habitat/pull/1370) ([smurawski](https://github.com/smurawski))
- improve docs documentation and add run task to Makefile [\#1369](https://github.com/habitat-sh/habitat/pull/1369) ([reset](https://github.com/reset))
- improve docs documentation and add run task to Makefile [\#1369](https://github.com/habitat-sh/habitat/pull/1369) ([reset](https://github.com/reset))
- Add Linux only comment to -R flag in docs [\#1367](https://github.com/habitat-sh/habitat/pull/1367) ([reset](https://github.com/reset))
- Add Linux only comment to -R flag in docs [\#1367](https://github.com/habitat-sh/habitat/pull/1367) ([reset](https://github.com/reset))
- Move a manual build step to Makefile from README for www project [\#1366](https://github.com/habitat-sh/habitat/pull/1366) ([reset](https://github.com/reset))
- remove pkg-config hack from worker plan [\#1365](https://github.com/habitat-sh/habitat/pull/1365) ([reset](https://github.com/reset))
- Updating code snippets to remove sudo inline with hab 0.11 release [\#1362](https://github.com/habitat-sh/habitat/pull/1362) ([davidwrede](https://github.com/davidwrede))
- Update www docs with a few clarifications. [\#1359](https://github.com/habitat-sh/habitat/pull/1359) ([raskchanky](https://github.com/raskchanky))
- Rough draft of new Windows tutorial [\#1350](https://github.com/habitat-sh/habitat/pull/1350) ([davidwrede](https://github.com/davidwrede))

## Bug fixes

- No feedback when downloading hab studio [\#1361](https://github.com/habitat-sh/habitat/issues/1361)
- Docs: Link to missing wikipedia page [\#1352](https://github.com/habitat-sh/habitat/issues/1352)
- Habitat Studio doesn't work on paths with space inside [\#1338](https://github.com/habitat-sh/habitat/issues/1338)
- -R option is either mis documented, or not implemented [\#876](https://github.com/habitat-sh/habitat/issues/876)
- Quoted $studio\_config variable \#1338 [\#1376](https://github.com/habitat-sh/habitat/pull/1376) ([Albibek](https://github.com/Albibek))
- \[common\] use root cache key path for package install [\#1372](https://github.com/habitat-sh/habitat/pull/1372) ([chefsalim](https://github.com/chefsalim))

**Closed issues:**

- `hab config apply` does not work as expected [\#1355](https://github.com/habitat-sh/habitat/issues/1355)
- hab 0.10.2 doesn't copy keys inside the studio [\#1341](https://github.com/habitat-sh/habitat/issues/1341)
- \[discussion\] Should template files have an extension? [\#1183](https://github.com/habitat-sh/habitat/issues/1183)

**Merged pull requests:**

- Enable `HAB\_DOCKER\_OPS` environment variable. [\#1410](https://github.com/habitat-sh/habitat/pull/1410) ([Hoverbear](https://github.com/Hoverbear))
- Improve timeout reporting [\#1401](https://github.com/habitat-sh/habitat/pull/1401) ([reset](https://github.com/reset))
- Hide habitat summit banner [\#1400](https://github.com/habitat-sh/habitat/pull/1400) ([ryankeairns](https://github.com/ryankeairns))
- Updated Mac instructions for building Hab [\#1396](https://github.com/habitat-sh/habitat/pull/1396) ([jjasghar](https://github.com/jjasghar))
- Make `fix\_interpreter` resolve symbolic links [\#1386](https://github.com/habitat-sh/habitat/pull/1386) ([smith](https://github.com/smith))
- Reload the origin after private key upload so it shows up right away [\#1385](https://github.com/habitat-sh/habitat/pull/1385) ([raskchanky](https://github.com/raskchanky))
- Document `download\_file` function [\#1382](https://github.com/habitat-sh/habitat/pull/1382) ([smith](https://github.com/smith))
- Change how we check origin membership to make the Members tab show up [\#1380](https://github.com/habitat-sh/habitat/pull/1380) ([raskchanky](https://github.com/raskchanky))
- Fix some bugs with linking packages to repos [\#1379](https://github.com/habitat-sh/habitat/pull/1379) ([raskchanky](https://github.com/raskchanky))
- Add Gossip to Swim, creating a Butterfly [\#1377](https://github.com/habitat-sh/habitat/pull/1377) ([adamhjk](https://github.com/adamhjk))
- Improve user output - show Docker pull progress on studio enter [\#1374](https://github.com/habitat-sh/habitat/pull/1374) ([chefsalim](https://github.com/chefsalim))
- \[JEX-346\] Unify the Linux and MacOS install scripts [\#1364](https://github.com/habitat-sh/habitat/pull/1364) ([jtimberman](https://github.com/jtimberman))
- fix windows binary building on linux/vbox [\#1358](https://github.com/habitat-sh/habitat/pull/1358) ([mwrock](https://github.com/mwrock))
- Fix lots of bugs from the Angular 2 upgrade [\#1330](https://github.com/habitat-sh/habitat/pull/1330) ([raskchanky](https://github.com/raskchanky))

## [0.11.0](https://github.com/habitat-sh/habitat/tree/0.11.0) (10-14-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.10.2...0.11.0)

## Features & Enhancements

- Port Hab command-line tool to Windows [\#1192](https://github.com/habitat-sh/habitat/issues/1192)
- Warn when docker-based studio is out of date [\#1167](https://github.com/habitat-sh/habitat/issues/1167)
- Version ordering seems to be alpha sorted and needs to following some kind of versioning comparison scheme [\#1090](https://github.com/habitat-sh/habitat/issues/1090)
- Package names with a dot generate invalid default group [\#1089](https://github.com/habitat-sh/habitat/issues/1089)
- Check write permissions before attempting to download [\#960](https://github.com/habitat-sh/habitat/issues/960)
- Sodiumoxide cargo config override can be removed [\#952](https://github.com/habitat-sh/habitat/issues/952)
- be able to modify and rebuild from plan.sh without rerunning a full build. [\#927](https://github.com/habitat-sh/habitat/issues/927)
- \[builder-web\] Upgrade to new Angular 2 Router [\#885](https://github.com/habitat-sh/habitat/issues/885)
- \[builder-web\] Upgrade to Angular 2 RC [\#884](https://github.com/habitat-sh/habitat/issues/884)
- Really fix the admin status check on Windows this time [\#1342](https://github.com/habitat-sh/habitat/pull/1342) ([smurawski](https://github.com/smurawski))
- Make Admin/Elevated Admin Token == root for hab commands [\#1337](https://github.com/habitat-sh/habitat/pull/1337) ([smurawski](https://github.com/smurawski))
- \[ci\] Support `$STEAM\_ROLLER` to force a build unconditionally. [\#1335](https://github.com/habitat-sh/habitat/pull/1335) ([fnichol](https://github.com/fnichol))
- \[ci\] Remove unused and redundant cache directories. [\#1334](https://github.com/habitat-sh/habitat/pull/1334) ([fnichol](https://github.com/fnichol))
- \[ci\] Drop requirement for nightly cargo installation. [\#1333](https://github.com/habitat-sh/habitat/pull/1333) ([fnichol](https://github.com/fnichol))
- \[ci\] Properly detect version of rustfmt for install logic. [\#1332](https://github.com/habitat-sh/habitat/pull/1332) ([fnichol](https://github.com/fnichol))
- \[hab\] Attempt to call `sudo` on `hab studio \*` for non-root users. [\#1329](https://github.com/habitat-sh/habitat/pull/1329) ([fnichol](https://github.com/fnichol))
- Remove vendored sodiumoxide and libsodium-sys from components [\#1326](https://github.com/habitat-sh/habitat/pull/1326) ([reset](https://github.com/reset))
- Detect, warn, & exit on operations that require root/admin permissions. [\#1325](https://github.com/habitat-sh/habitat/pull/1325) ([fnichol](https://github.com/fnichol))
- Set sensible defaults for pkg\_pconfig\_dirs plan variable [\#1323](https://github.com/habitat-sh/habitat/pull/1323) ([reset](https://github.com/reset))
- Improve error messages around Docker [\#1322](https://github.com/habitat-sh/habitat/pull/1322) ([raskchanky](https://github.com/raskchanky))
- \[hab\] Refactor constants to remove unused warns in studio subcmd. [\#1321](https://github.com/habitat-sh/habitat/pull/1321) ([fnichol](https://github.com/fnichol))
- \[hab\] Retire `core/hab-dynamic` Plan. [\#1320](https://github.com/habitat-sh/habitat/pull/1320) ([fnichol](https://github.com/fnichol))
- Updating tutorial content and example code [\#1319](https://github.com/habitat-sh/habitat/pull/1319) ([davidwrede](https://github.com/davidwrede))
- Add terminfo to studio for colorization [\#1318](https://github.com/habitat-sh/habitat/pull/1318) ([smacfarlane](https://github.com/smacfarlane))
- Streamline Bintray release instructions in RELEASE.md. [\#1316](https://github.com/habitat-sh/habitat/pull/1316) ([fnichol](https://github.com/fnichol))
- Streamline Bintray release instructions in RELEASE.md. [\#1316](https://github.com/habitat-sh/habitat/pull/1316) ([fnichol](https://github.com/fnichol))
- `hab studio enter` on Windows via Docker! [\#1309](https://github.com/habitat-sh/habitat/pull/1309) ([smurawski](https://github.com/smurawski))
- Remove unconditional reference to sudo\(8\). [\#1304](https://github.com/habitat-sh/habitat/pull/1304) ([juliandunn](https://github.com/juliandunn))
- Update to latest ZeroMQ crate [\#1252](https://github.com/habitat-sh/habitat/pull/1252) ([reset](https://github.com/reset))

## Bug fixes

- "Invalid status provided" on Habitat 0.10.1 because it tries to send data to Google [\#1306](https://github.com/habitat-sh/habitat/issues/1306)
- hab pkg build returns code 0 on error [\#1285](https://github.com/habitat-sh/habitat/issues/1285)
- Generated svc `run` script assumes binaries are on path [\#1268](https://github.com/habitat-sh/habitat/issues/1268)
- Check filetype of static/run in svc directory before writing to [\#1267](https://github.com/habitat-sh/habitat/issues/1267)
- Overriding nested \(table\) config vars clobbers defaults [\#1184](https://github.com/habitat-sh/habitat/issues/1184)
- hab pkg upload fails if public key isn't present in /hab/cache/keys [\#1172](https://github.com/habitat-sh/habitat/issues/1172)
- Version ordering seems to be alpha sorted and needs to following some kind of versioning comparison scheme [\#1090](https://github.com/habitat-sh/habitat/issues/1090)
- Package names with a dot generate invalid default group [\#1089](https://github.com/habitat-sh/habitat/issues/1089)
- Don't restart child process if reconfigure hook is defined [\#1336](https://github.com/habitat-sh/habitat/pull/1336) ([metadave](https://github.com/metadave))
- \[studio\] Properly propagate the exit code when `build` fails. [\#1331](https://github.com/habitat-sh/habitat/pull/1331) ([fnichol](https://github.com/fnichol))
- replacing execv in exec.rs with std::process::Command [\#1324](https://github.com/habitat-sh/habitat/pull/1324) ([rrxtns](https://github.com/rrxtns))

**Closed issues:**

- Depot fails to link package to GitHub repo due to CORS issue [\#1344](https://github.com/habitat-sh/habitat/issues/1344)
- Unauthorized when uploading keys/packages to depot [\#1343](https://github.com/habitat-sh/habitat/issues/1343)
- Keys not loaded properly when installing .hart locally [\#1317](https://github.com/habitat-sh/habitat/issues/1317)

**Merged pull requests:**

- End to end cleanup of windows binary building [\#1357](https://github.com/habitat-sh/habitat/pull/1357) ([mwrock](https://github.com/mwrock))
- revert windows download button until bintray publish is fleshed out [\#1354](https://github.com/habitat-sh/habitat/pull/1354) ([mwrock](https://github.com/mwrock))
- 0.11.0 [\#1353](https://github.com/habitat-sh/habitat/pull/1353) ([reset](https://github.com/reset))
- Add windows binaries to available downloads [\#1348](https://github.com/habitat-sh/habitat/pull/1348) ([mwrock](https://github.com/mwrock))
- \[core\] Check invalid characters in package name on upload [\#1347](https://github.com/habitat-sh/habitat/pull/1347) ([chefsalim](https://github.com/chefsalim))
- \[build\] Validate characters in plan name [\#1345](https://github.com/habitat-sh/habitat/pull/1345) ([chefsalim](https://github.com/chefsalim))
- Fix build step in builder-worker plan [\#1328](https://github.com/habitat-sh/habitat/pull/1328) ([reset](https://github.com/reset))
- Bump VERSION to 0.11.0-dev. [\#1315](https://github.com/habitat-sh/habitat/pull/1315) ([fnichol](https://github.com/fnichol))
- SWIM as an independent library [\#1310](https://github.com/habitat-sh/habitat/pull/1310) ([adamhjk](https://github.com/adamhjk))

## [0.10.2](https://github.com/habitat-sh/habitat/tree/0.10.2) (09-30-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.10.1...0.10.2)

## Bug fixes

- Unable to update config values specified in default.toml - 0.10.1 [\#1307](https://github.com/habitat-sh/habitat/issues/1307)
- The command "hab sup config package" should output the default.toml of the package. [\#1301](https://github.com/habitat-sh/habitat/issues/1301)
- fix merging top-level config values [\#1308](https://github.com/habitat-sh/habitat/pull/1308) ([robbkidd](https://github.com/robbkidd))

**Merged pull requests:**

- Adding text on updating services through env var + title changes [\#1311](https://github.com/habitat-sh/habitat/pull/1311) ([davidwrede](https://github.com/davidwrede))
- Start 0.11.0-dev [\#1305](https://github.com/habitat-sh/habitat/pull/1305) ([reset](https://github.com/reset))

## [0.10.1](https://github.com/habitat-sh/habitat/tree/0.10.1) (09-29-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.10.0...0.10.1)

## Features & Enhancements

- Drop Cargo nightly requirement as of Rust 1.12.0 release. [\#1299](https://github.com/habitat-sh/habitat/pull/1299) ([fnichol](https://github.com/fnichol))

## Bug fixes

- hab won't install packages if /hab/cache/artifacts isn't present [\#1291](https://github.com/habitat-sh/habitat/issues/1291)
- Fix issue entering studios in release versions of hab [\#1302](https://github.com/habitat-sh/habitat/pull/1302) ([reset](https://github.com/reset))

**Merged pull requests:**

- 0.10.1 [\#1303](https://github.com/habitat-sh/habitat/pull/1303) ([reset](https://github.com/reset))
- Start 0.11.0-dev [\#1300](https://github.com/habitat-sh/habitat/pull/1300) ([reset](https://github.com/reset))

## [0.10.0](https://github.com/habitat-sh/habitat/tree/0.10.0) (09-29-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/BLDR-0.10.0...0.10.0)

## Features & Enhancements

- \[documentation\] Testing locally compiled dependencies without `hab pkg upload`-ing [\#1273](https://github.com/habitat-sh/habitat/issues/1273)
- Move Homu to bots.habitat.sh [\#1113](https://github.com/habitat-sh/habitat/issues/1113)
- Bot Monitoring [\#1111](https://github.com/habitat-sh/habitat/issues/1111)
- \[plans\] Curl plan should support/find CA certificates out of the box [\#718](https://github.com/habitat-sh/habitat/issues/718)
- Add all deps to the chocolatey package of hab dev build for Windows [\#1295](https://github.com/habitat-sh/habitat/pull/1295) ([smurawski](https://github.com/smurawski))
- Support windows command resolution [\#1290](https://github.com/habitat-sh/habitat/pull/1290) ([smurawski](https://github.com/smurawski))
- Remove the Waffle badge as we aren't currently using it. [\#1288](https://github.com/habitat-sh/habitat/pull/1288) ([juliandunn](https://github.com/juliandunn))
- Add step to BUILDING for sourcing cargo env to fix make error [\#1286](https://github.com/habitat-sh/habitat/pull/1286) ([robbkidd](https://github.com/robbkidd))
- Adding a nuspec template for  [\#1283](https://github.com/habitat-sh/habitat/pull/1283) ([smurawski](https://github.com/smurawski))
- Adding a build script for the Windows `hab` binary and AppVeyor Integration [\#1279](https://github.com/habitat-sh/habitat/pull/1279) ([smurawski](https://github.com/smurawski))
- move platform dependent calls from hab to core [\#1264](https://github.com/habitat-sh/habitat/pull/1264) ([smurawski](https://github.com/smurawski))
- Cleaning up the windows build docs [\#1262](https://github.com/habitat-sh/habitat/pull/1262) ([smurawski](https://github.com/smurawski))
- \[hab/mac\] Prompt for core origin key when performing a Mac hab release. [\#1258](https://github.com/habitat-sh/habitat/pull/1258) ([fnichol](https://github.com/fnichol))
- \[devshell\] Use rustup in Dockerfile. [\#1257](https://github.com/habitat-sh/habitat/pull/1257) ([fnichol](https://github.com/fnichol))
- Iterate on plans with --config-from [\#1256](https://github.com/habitat-sh/habitat/pull/1256) ([adamhjk](https://github.com/adamhjk))
- `pkg\_source` is required [\#1254](https://github.com/habitat-sh/habitat/pull/1254) ([nathenharvey](https://github.com/nathenharvey))
- Convert supervisor config to global singleton [\#1246](https://github.com/habitat-sh/habitat/pull/1246) ([adamhjk](https://github.com/adamhjk))

## Bug fixes

- `hab pkg hash` doesn't work with filenames with commas [\#1151](https://github.com/habitat-sh/habitat/issues/1151)
- service\_config::toml\_merge handles toml::Tables \(rebased w/ changed order\) [\#1294](https://github.com/habitat-sh/habitat/pull/1294) ([robbkidd](https://github.com/robbkidd))
- \[hab\] Fix missing artifact cache directory using `FS\_ROOT` env var. [\#1287](https://github.com/habitat-sh/habitat/pull/1287) ([fnichol](https://github.com/fnichol))
- \[Makefile\] Repair `serve-docs` target & support Docker for Mac. [\#1259](https://github.com/habitat-sh/habitat/pull/1259) ([fnichol](https://github.com/fnichol))

**Closed issues:**

- Unclear message when token has incorrect scope [\#1105](https://github.com/habitat-sh/habitat/issues/1105)

**Merged pull requests:**

- Release 0.10.0 [\#1298](https://github.com/habitat-sh/habitat/pull/1298) ([reset](https://github.com/reset))
- Fix bug with inclusion of whitespace in VERSION constant [\#1297](https://github.com/habitat-sh/habitat/pull/1297) ([reset](https://github.com/reset))
- Fix text alignment by removing one word from copy. [\#1296](https://github.com/habitat-sh/habitat/pull/1296) ([juliandunn](https://github.com/juliandunn))
- Require less broad OAuth scopes [\#1293](https://github.com/habitat-sh/habitat/pull/1293) ([reset](https://github.com/reset))
- \[WIP\] Ensure the hab versions inside and outside of a studio match [\#1292](https://github.com/habitat-sh/habitat/pull/1292) ([raskchanky](https://github.com/raskchanky))
- Request full user scope on web login [\#1289](https://github.com/habitat-sh/habitat/pull/1289) ([reset](https://github.com/reset))
- Bldr-0.11.0-dev [\#1284](https://github.com/habitat-sh/habitat/pull/1284) ([reset](https://github.com/reset))
- Improve messaging for failed login due to missing OAuth scope\(s\) [\#1282](https://github.com/habitat-sh/habitat/pull/1282) ([reset](https://github.com/reset))
- Update to Windows Build Docs [\#1281](https://github.com/habitat-sh/habitat/pull/1281) ([smurawski](https://github.com/smurawski))
- Improve client & server error reporting [\#1277](https://github.com/habitat-sh/habitat/pull/1277) ([reset](https://github.com/reset))
- A few small copy edits for the website home page [\#1276](https://github.com/habitat-sh/habitat/pull/1276) ([ryankeairns](https://github.com/ryankeairns))
- Add Steven as a maintainer [\#1275](https://github.com/habitat-sh/habitat/pull/1275) ([smurawski](https://github.com/smurawski))
- Updating linux hab setup instructions and minor copy edits [\#1274](https://github.com/habitat-sh/habitat/pull/1274) ([davidwrede](https://github.com/davidwrede))
- Use new sentinel testing branches [\#1272](https://github.com/habitat-sh/habitat/pull/1272) ([adamhjk](https://github.com/adamhjk))
- Update documentation references for user scope [\#1271](https://github.com/habitat-sh/habitat/pull/1271) ([jtimberman](https://github.com/jtimberman))
- Stub out uname and effective\_uid to make hab run [\#1270](https://github.com/habitat-sh/habitat/pull/1270) ([smurawski](https://github.com/smurawski))
- \[hab\] Read user's keys when running `hab pkg build` with `sudo`. [\#1269](https://github.com/habitat-sh/habitat/pull/1269) ([fnichol](https://github.com/fnichol))
- Builder 0.10.0 [\#1266](https://github.com/habitat-sh/habitat/pull/1266) ([reset](https://github.com/reset))
- branches that have test\_development-.\* will run tests [\#1261](https://github.com/habitat-sh/habitat/pull/1261) ([metadave](https://github.com/metadave))
- Update Admin, API, and Depot route definitions with names [\#1260](https://github.com/habitat-sh/habitat/pull/1260) ([reset](https://github.com/reset))
- Update Cargo dependencies [\#1255](https://github.com/habitat-sh/habitat/pull/1255) ([adamhjk](https://github.com/adamhjk))
- Bump VERSION to 0.10.0-dev. [\#1251](https://github.com/habitat-sh/habitat/pull/1251) ([fnichol](https://github.com/fnichol))
- Home page copy updates [\#1247](https://github.com/habitat-sh/habitat/pull/1247) ([ryankeairns](https://github.com/ryankeairns))
- Upgrade to Angular 2 RC-5 [\#1213](https://github.com/habitat-sh/habitat/pull/1213) ([raskchanky](https://github.com/raskchanky))

## [BLDR-0.10.0](https://github.com/habitat-sh/habitat/tree/BLDR-0.10.0) (09-16-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.9.3...BLDR-0.10.0)

**Merged pull requests:**

- Update clap to fix hashing file names with commas. [\#1250](https://github.com/habitat-sh/habitat/pull/1250) ([raskchanky](https://github.com/raskchanky))
- Plumb feature flags into builder-web [\#1249](https://github.com/habitat-sh/habitat/pull/1249) ([reset](https://github.com/reset))

## [0.9.3](https://github.com/habitat-sh/habitat/tree/0.9.3) (09-16-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.9.2...0.9.3)

## Bug fixes

- \[hab\] Fix UI import for non-Linux \(i.e. Mac\) builds. [\#1245](https://github.com/habitat-sh/habitat/pull/1245) ([fnichol](https://github.com/fnichol))

**Merged pull requests:**

- version bump and changelog update for 0.9.3 [\#1248](https://github.com/habitat-sh/habitat/pull/1248) ([metadave](https://github.com/metadave))
- Improve logging/handling when receiving an unexpected route reply [\#1244](https://github.com/habitat-sh/habitat/pull/1244) ([reset](https://github.com/reset))

## [0.9.2](https://github.com/habitat-sh/habitat/tree/0.9.2) (09-15-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.9.1...0.9.2)

## Features & Enhancements

- Update Getting Started tutorial to use Docker for Mac instead of Docker Toolbox [\#1130](https://github.com/habitat-sh/habitat/issues/1130)
- \[hab\] Final UI additions to control coloring/formatting for errors and analytics [\#1239](https://github.com/habitat-sh/habitat/pull/1239) ([fnichol](https://github.com/fnichol))
- \[hab,sup\] Add formatted output & input methods to `UI`. [\#1237](https://github.com/habitat-sh/habitat/pull/1237) ([fnichol](https://github.com/fnichol))
- \[hab,sup\] Add `UI\#progress\(\)` method to control progress bar behavior. [\#1233](https://github.com/habitat-sh/habitat/pull/1233) ([fnichol](https://github.com/fnichol))
- Clean the middleman build before building [\#1232](https://github.com/habitat-sh/habitat/pull/1232) ([reset](https://github.com/reset))
- Just enough code to make the core crate tests pass [\#1231](https://github.com/habitat-sh/habitat/pull/1231) ([smurawski](https://github.com/smurawski))
- \[hab,sup\] Add common UI subsystem to manage output display. [\#1228](https://github.com/habitat-sh/habitat/pull/1228) ([fnichol](https://github.com/fnichol))
- Use `bodyparser::Struct` to parse request bodies in HTTP Gateways [\#1227](https://github.com/habitat-sh/habitat/pull/1227) ([reset](https://github.com/reset))
- Revert back to typed headers in HTTP gateways [\#1226](https://github.com/habitat-sh/habitat/pull/1226) ([reset](https://github.com/reset))
- add README & Makefile to www project [\#1223](https://github.com/habitat-sh/habitat/pull/1223) ([reset](https://github.com/reset))
- Getting started tutorial split by OS, UX updates, and more [\#1220](https://github.com/habitat-sh/habitat/pull/1220) ([davidwrede](https://github.com/davidwrede))
- add/improve make targets for running & building builder services [\#1219](https://github.com/habitat-sh/habitat/pull/1219) ([reset](https://github.com/reset))
- Make Authenticated middleware self contained [\#1217](https://github.com/habitat-sh/habitat/pull/1217) ([reset](https://github.com/reset))
- \[hab\] Split code into lib & bin modules [\#1203](https://github.com/habitat-sh/habitat/pull/1203) ([fnichol](https://github.com/fnichol))
- Adding package search from the CLI. Fixes \#1159 [\#1201](https://github.com/habitat-sh/habitat/pull/1201) ([nsdavidson](https://github.com/nsdavidson))

## Bug fixes

- build fails if pkg\_source is blank [\#870](https://github.com/habitat-sh/habitat/issues/870)
- \[Cargo\] Update Cargo.lock from new Windows dependency additions. [\#1238](https://github.com/habitat-sh/habitat/pull/1238) ([fnichol](https://github.com/fnichol))
- Move buffer trimming to the final buffer [\#1225](https://github.com/habitat-sh/habitat/pull/1225) ([adamhjk](https://github.com/adamhjk))
- Quick fix for sorting package identifiers returned by the Depot API [\#1221](https://github.com/habitat-sh/habitat/pull/1221) ([reset](https://github.com/reset))

**Closed issues:**

- init hook returning invalid exit code [\#1222](https://github.com/habitat-sh/habitat/issues/1222)
- 3e [\#1215](https://github.com/habitat-sh/habitat/issues/1215)

**Merged pull requests:**

- Static directory permissions [\#1241](https://github.com/habitat-sh/habitat/pull/1241) ([jtimberman](https://github.com/jtimberman))
- Always render run script with chpst [\#1240](https://github.com/habitat-sh/habitat/pull/1240) ([jtimberman](https://github.com/jtimberman))
- windows terminal support tweaks [\#1235](https://github.com/habitat-sh/habitat/pull/1235) ([smurawski](https://github.com/smurawski))
- On windows, order matters - case does not [\#1234](https://github.com/habitat-sh/habitat/pull/1234) ([smurawski](https://github.com/smurawski))
- Add Habitat Community Summit text to global message [\#1230](https://github.com/habitat-sh/habitat/pull/1230) ([magwalk](https://github.com/magwalk))
- Adding download button to hero per AB test results [\#1218](https://github.com/habitat-sh/habitat/pull/1218) ([ryankeairns](https://github.com/ryankeairns))
- docs\(README.md\) improve readability of commands [\#1216](https://github.com/habitat-sh/habitat/pull/1216) ([d2s](https://github.com/d2s))
- Test for required metadata keys before build [\#1214](https://github.com/habitat-sh/habitat/pull/1214) ([miketheman](https://github.com/miketheman))
- Fix deserialization error on InstaSet datastore write [\#1212](https://github.com/habitat-sh/habitat/pull/1212) ([reset](https://github.com/reset))
- Fix package specification for cmake [\#1211](https://github.com/habitat-sh/habitat/pull/1211) ([smurawski](https://github.com/smurawski))
- depot/builder-api refactors [\#1210](https://github.com/habitat-sh/habitat/pull/1210) ([reset](https://github.com/reset))
- Update VERSION to the next one [\#1209](https://github.com/habitat-sh/habitat/pull/1209) ([raskchanky](https://github.com/raskchanky))
- Integrate projects with the package details page [\#1157](https://github.com/habitat-sh/habitat/pull/1157) ([raskchanky](https://github.com/raskchanky))

## [0.9.1](https://github.com/habitat-sh/habitat/tree/0.9.1) (09-01-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.9.0...0.9.1)

## Features & Enhancements

- `hab pkg search` or similar functionality [\#1159](https://github.com/habitat-sh/habitat/issues/1159)
- Document required structure of run hook [\#989](https://github.com/habitat-sh/habitat/issues/989)
- \[ci/lint\] Fix hard-coded rustfmt version in lint.sh. [\#1206](https://github.com/habitat-sh/habitat/pull/1206) ([fnichol](https://github.com/fnichol))
- \[ci/lint\] Update rustfmt version parsing logic. [\#1205](https://github.com/habitat-sh/habitat/pull/1205) ([fnichol](https://github.com/fnichol))
- \[ci/rustfmt\] Uninstall then install rustfmt when upgrading. [\#1204](https://github.com/habitat-sh/habitat/pull/1204) ([fnichol](https://github.com/fnichol))
- \[ci/rustfmt\] Force install newer version of rustfmt when required. [\#1202](https://github.com/habitat-sh/habitat/pull/1202) ([fnichol](https://github.com/fnichol))
- \[ci/rustfmt\] Upgrade to lint with rustfmt 0.6.0. [\#1200](https://github.com/habitat-sh/habitat/pull/1200) ([fnichol](https://github.com/fnichol))
- Remove extra word [\#1191](https://github.com/habitat-sh/habitat/pull/1191) ([ksexton](https://github.com/ksexton))
- Adds access token scope to CLI [\#1189](https://github.com/habitat-sh/habitat/pull/1189) ([freethejazz](https://github.com/freethejazz))
- touchup backline plan file [\#1166](https://github.com/habitat-sh/habitat/pull/1166) ([reset](https://github.com/reset))
- Adding routes for downloading packages and keys from views [\#1141](https://github.com/habitat-sh/habitat/pull/1141) ([nsdavidson](https://github.com/nsdavidson))
- GitHub building & Admin Gateway [\#1070](https://github.com/habitat-sh/habitat/pull/1070) ([reset](https://github.com/reset))

## Bug fixes

- Corporate proxy not working on Mac [\#1180](https://github.com/habitat-sh/habitat/issues/1180)
- setuid bit is lost upon install [\#1175](https://github.com/habitat-sh/habitat/issues/1175)
- Habitat tutorial fails to run init hook due to permissions problems [\#1168](https://github.com/habitat-sh/habitat/issues/1168)
- build error: 'ssl:error:SslError' is undefined [\#1155](https://github.com/habitat-sh/habitat/issues/1155)
- Non-existent hab user/group results in `Failed to set permissions` error [\#755](https://github.com/habitat-sh/habitat/issues/755)
- \[http-client\] Add missing colon on Host header when connecting to proxy. [\#1199](https://github.com/habitat-sh/habitat/pull/1199) ([fnichol](https://github.com/fnichol))
- \[core\] Use 'Owner' and 'Permissions' during unpack [\#1187](https://github.com/habitat-sh/habitat/pull/1187) ([fujin](https://github.com/fujin))
- update mytutorialapp to work with Habitat 0.9.0 [\#1169](https://github.com/habitat-sh/habitat/pull/1169) ([metadave](https://github.com/metadave))
- Quote no\_proxy correctly [\#1162](https://github.com/habitat-sh/habitat/pull/1162) ([adamhjk](https://github.com/adamhjk))

**Closed issues:**

- Can't enter studio on Mac Docker [\#1197](https://github.com/habitat-sh/habitat/issues/1197)
- Provide a `fix\_interpreter\_unsafe` function [\#1182](https://github.com/habitat-sh/habitat/issues/1182)
- hab studio enter fails on mac os x [\#1178](https://github.com/habitat-sh/habitat/issues/1178)
- differing versions of `core/openssl` cause conflicts between `core/python2` and `core/postgresql` [\#1037](https://github.com/habitat-sh/habitat/issues/1037)

**Merged pull requests:**

- Releasing 0.9.1 [\#1208](https://github.com/habitat-sh/habitat/pull/1208) ([raskchanky](https://github.com/raskchanky))
- Start of Building on Windows doc [\#1198](https://github.com/habitat-sh/habitat/pull/1198) ([smurawski](https://github.com/smurawski))
- builder-api should start as root/root [\#1196](https://github.com/habitat-sh/habitat/pull/1196) ([jtimberman](https://github.com/jtimberman))
- Extract linux/mac specific behavior in the habitat core crate [\#1193](https://github.com/habitat-sh/habitat/pull/1193) ([smurawski](https://github.com/smurawski))
- Set the pkg user/group for builder-api-proxy [\#1190](https://github.com/habitat-sh/habitat/pull/1190) ([jtimberman](https://github.com/jtimberman))
- Updated link to building packages topic [\#1186](https://github.com/habitat-sh/habitat/pull/1186) ([davidwrede](https://github.com/davidwrede))
- update tutorial shasum for v0.9.0 [\#1181](https://github.com/habitat-sh/habitat/pull/1181) ([metadave](https://github.com/metadave))
- Updated plan example and fixed link to core plans repo. [\#1179](https://github.com/habitat-sh/habitat/pull/1179) ([davidwrede](https://github.com/davidwrede))
- updated and clarified valid service group names [\#1177](https://github.com/habitat-sh/habitat/pull/1177) ([davidwrede](https://github.com/davidwrede))
- \[docs\] Install cargo 0.13 nightly \(fixes \#1155\) [\#1176](https://github.com/habitat-sh/habitat/pull/1176) ([chetan](https://github.com/chetan))
- files in a packages config directory are logged upon parse failure [\#1171](https://github.com/habitat-sh/habitat/pull/1171) ([metadave](https://github.com/metadave))
- Start 0.10.0-dev [\#1165](https://github.com/habitat-sh/habitat/pull/1165) ([reset](https://github.com/reset))
- Typo in "announcement" [\#1164](https://github.com/habitat-sh/habitat/pull/1164) ([juliandunn](https://github.com/juliandunn))
- Habitat publishing guide and program fixes [\#1163](https://github.com/habitat-sh/habitat/pull/1163) ([reset](https://github.com/reset))
- Add CLI UX principles to repo [\#1158](https://github.com/habitat-sh/habitat/pull/1158) ([ryankeairns](https://github.com/ryankeairns))
- Document use of exec in run hooks [\#1154](https://github.com/habitat-sh/habitat/pull/1154) ([mivok](https://github.com/mivok))

## [0.9.0](https://github.com/habitat-sh/habitat/tree/0.9.0) (08-15-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.8.0...0.9.0)

## Features & Enhancements

- Documentation fix - plan syntax [\#1147](https://github.com/habitat-sh/habitat/issues/1147)
- Building with a shared cargo dir? [\#1129](https://github.com/habitat-sh/habitat/issues/1129)
- Please add search to the habitat docs page [\#1098](https://github.com/habitat-sh/habitat/issues/1098)
- Please add search to the habitat docs page [\#1098](https://github.com/habitat-sh/habitat/issues/1098)
- Most executables and libraries are not stripped \(feature not bug?\) [\#1066](https://github.com/habitat-sh/habitat/issues/1066)
- Failed to build habat on CentOS 7.2 [\#1063](https://github.com/habitat-sh/habitat/issues/1063)
- Docs needed for do\_check callback [\#1039](https://github.com/habitat-sh/habitat/issues/1039)
- \[hab\] Add an optional arg to install.sh to specify a version of hab. [\#1149](https://github.com/habitat-sh/habitat/pull/1149) ([fnichol](https://github.com/fnichol))
- \[hab\] Mac build improvements [\#1144](https://github.com/habitat-sh/habitat/pull/1144) ([fnichol](https://github.com/fnichol))
- Use a consistent template for Rust component Plans. [\#1142](https://github.com/habitat-sh/habitat/pull/1142) ([fnichol](https://github.com/fnichol))
- Rewrite package install strategy. [\#1093](https://github.com/habitat-sh/habitat/pull/1093) ([fnichol](https://github.com/fnichol))

## Bug fixes

- \[hab-sup\] Handlebars templates don't allow `-` [\#1117](https://github.com/habitat-sh/habitat/issues/1117)
- cfg variables don't expand inside Handlebars each block helper [\#1083](https://github.com/habitat-sh/habitat/issues/1083)
- build errors on blake2bsums for files with spaces in them [\#1065](https://github.com/habitat-sh/habitat/issues/1065)
- Wildcard fix\_interpreter throws error if the path contains directories [\#1045](https://github.com/habitat-sh/habitat/issues/1045)
- Be able to specify that a service wants a SIGHUP for reconfigure [\#987](https://github.com/habitat-sh/habitat/issues/987)
- studio plan is dependent on removed package - hab-static [\#974](https://github.com/habitat-sh/habitat/issues/974)
- Rename remaining references of "view" to "channel" [\#1153](https://github.com/habitat-sh/habitat/pull/1153) ([reset](https://github.com/reset))
- \[hab,hab-sup\] Fix progress bar/newline rendering issue. [\#1125](https://github.com/habitat-sh/habitat/pull/1125) ([fnichol](https://github.com/fnichol))

**Closed issues:**

- Invalid Status provided trying to build Habitat on Mac [\#1134](https://github.com/habitat-sh/habitat/issues/1134)
- Failed to build habitat on ubuntu 14.04 [\#1058](https://github.com/habitat-sh/habitat/issues/1058)

**Merged pull requests:**

- update build docs [\#1156](https://github.com/habitat-sh/habitat/pull/1156) ([metadave](https://github.com/metadave))
- CI will run specs contained in ./test [\#1152](https://github.com/habitat-sh/habitat/pull/1152) ([metadave](https://github.com/metadave))
- Rename depot/depot-client to builder-depot/builder-depot-client [\#1150](https://github.com/habitat-sh/habitat/pull/1150) ([reset](https://github.com/reset))
- specify that pkg\_description is not optional in www docs [\#1148](https://github.com/habitat-sh/habitat/pull/1148) ([metadave](https://github.com/metadave))
- replace shellouts with libc calls where possible [\#1143](https://github.com/habitat-sh/habitat/pull/1143) ([metadave](https://github.com/metadave))
- Updating run-api.sh to work with new target location [\#1140](https://github.com/habitat-sh/habitat/pull/1140) ([nsdavidson](https://github.com/nsdavidson))
- Clarify how to pass configuration updates [\#1139](https://github.com/habitat-sh/habitat/pull/1139) ([bdangit](https://github.com/bdangit))
- Use Cargo "workspaces" to manage a project-wide Cargo.lock file [\#1136](https://github.com/habitat-sh/habitat/pull/1136) ([reset](https://github.com/reset))
- Projects can be deleted via the API [\#1135](https://github.com/habitat-sh/habitat/pull/1135) ([raskchanky](https://github.com/raskchanky))
- add Habitat testing framework [\#1133](https://github.com/habitat-sh/habitat/pull/1133) ([metadave](https://github.com/metadave))
- Native Docker for Mac is now released. [\#1128](https://github.com/habitat-sh/habitat/pull/1128) ([juliandunn](https://github.com/juliandunn))
- Copy no\_proxy settings to studio [\#1124](https://github.com/habitat-sh/habitat/pull/1124) ([stephenbm](https://github.com/stephenbm))
- Fetch projects using the API [\#1122](https://github.com/habitat-sh/habitat/pull/1122) ([raskchanky](https://github.com/raskchanky))
- Adds optimizely snippet to head [\#1121](https://github.com/habitat-sh/habitat/pull/1121) ([ryankeairns](https://github.com/ryankeairns))
- update handlebars to 0.20.1 [\#1119](https://github.com/habitat-sh/habitat/pull/1119) ([metadave](https://github.com/metadave))
- Update for Terraform 0.7 [\#1118](https://github.com/habitat-sh/habitat/pull/1118) ([reset](https://github.com/reset))
- Fix a bug with the origin in project creation [\#1115](https://github.com/habitat-sh/habitat/pull/1115) ([raskchanky](https://github.com/raskchanky))
- Rename 'views' to 'channels' [\#1114](https://github.com/habitat-sh/habitat/pull/1114) ([reset](https://github.com/reset))
- Adds trailing slash to fix search bug [\#1112](https://github.com/habitat-sh/habitat/pull/1112) ([magwalk](https://github.com/magwalk))
- Add Google Custom Search to Docs [\#1110](https://github.com/habitat-sh/habitat/pull/1110) ([magwalk](https://github.com/magwalk))
- Add myself as a maintainer [\#1109](https://github.com/habitat-sh/habitat/pull/1109) ([raskchanky](https://github.com/raskchanky))
- \[WIP\] Update project API on the frontend [\#1108](https://github.com/habitat-sh/habitat/pull/1108) ([raskchanky](https://github.com/raskchanky))
- Start working on projects front end [\#1107](https://github.com/habitat-sh/habitat/pull/1107) ([smith](https://github.com/smith))
- Bump handlebars to 0.19.0 [\#1106](https://github.com/habitat-sh/habitat/pull/1106) ([reset](https://github.com/reset))
- \[core\] Set mtime on files while unpacking archive [\#1100](https://github.com/habitat-sh/habitat/pull/1100) ([stevendanna](https://github.com/stevendanna))
- try the old bots once again :| [\#1097](https://github.com/habitat-sh/habitat/pull/1097) ([smith](https://github.com/smith))
- api-shell make target [\#1095](https://github.com/habitat-sh/habitat/pull/1095) ([smith](https://github.com/smith))
- add application/json type to render\_package [\#1092](https://github.com/habitat-sh/habitat/pull/1092) ([lamont-granquist](https://github.com/lamont-granquist))
- Set the SSL\_CERT\_FILE for the director [\#1091](https://github.com/habitat-sh/habitat/pull/1091) ([jtimberman](https://github.com/jtimberman))
- Remove a leading space from the config file output [\#1088](https://github.com/habitat-sh/habitat/pull/1088) ([smith](https://github.com/smith))
- \[hab-studio\] fix key export/import err handling upon studio enter [\#1084](https://github.com/habitat-sh/habitat/pull/1084) ([metadave](https://github.com/metadave))
- fix Director env doc bug [\#1078](https://github.com/habitat-sh/habitat/pull/1078) ([metadave](https://github.com/metadave))
- Added missing 'hab pkg upload' documentation [\#1077](https://github.com/habitat-sh/habitat/pull/1077) ([mschygulla](https://github.com/mschygulla))
- \[hab-plan-build\] filenames with whitespace\(s\) breaks FILES metafile generation [\#1076](https://github.com/habitat-sh/habitat/pull/1076) ([metadave](https://github.com/metadave))
- \[hab-director\] per-service environment variables [\#1075](https://github.com/habitat-sh/habitat/pull/1075) ([metadave](https://github.com/metadave))
- fix\_interpreter: warn+skip on non-regular files [\#1073](https://github.com/habitat-sh/habitat/pull/1073) ([srenatus](https://github.com/srenatus))
- Fix broken links in the readme [\#1071](https://github.com/habitat-sh/habitat/pull/1071) ([tas50](https://github.com/tas50))

## [0.8.0](https://github.com/habitat-sh/habitat/tree/0.8.0) (07-08-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.7.0...0.8.0)

## Features & Enhancements

- pkg-config integration for plan-build [\#972](https://github.com/habitat-sh/habitat/issues/972)
- hab-depot is built in debug mode [\#968](https://github.com/habitat-sh/habitat/issues/968)
- CPPFLAGS not set by default [\#962](https://github.com/habitat-sh/habitat/issues/962)

## Bug fixes

- Cannot install a .hart file into another studio [\#1011](https://github.com/habitat-sh/habitat/issues/1011)
- Unused method in openssl plan [\#1007](https://github.com/habitat-sh/habitat/issues/1007)
- issues building c++ stuff because no CXXFLAGS [\#926](https://github.com/habitat-sh/habitat/issues/926)
- Uploading a package allowed when no key in Origin [\#895](https://github.com/habitat-sh/habitat/issues/895)
- \[builder-web\] Origins failing to load [\#894](https://github.com/habitat-sh/habitat/issues/894)
- Proxy support does not work [\#892](https://github.com/habitat-sh/habitat/issues/892)
- `\[err=2\] bad\_verification\_code` When signing without GitHub public email available [\#861](https://github.com/habitat-sh/habitat/issues/861)
- No gossip exchange if Organization is defined in Director service profile [\#754](https://github.com/habitat-sh/habitat/issues/754)

**Closed issues:**

- Documentation section links [\#1046](https://github.com/habitat-sh/habitat/issues/1046)
- Exporting Habitat build to run in non container environments [\#1027](https://github.com/habitat-sh/habitat/issues/1027)
- can't run `core/openvpn` from within an interactive Studio session [\#1026](https://github.com/habitat-sh/habitat/issues/1026)
- Incorrect argument to chpst in hab-plan-build.sh [\#1016](https://github.com/habitat-sh/habitat/issues/1016)
- Search for depot in the packages does not result in hab-depot [\#967](https://github.com/habitat-sh/habitat/issues/967)
- CMake doesn't install - hab-sup failed to set permissions error [\#951](https://github.com/habitat-sh/habitat/issues/951)
- hab pkg install does not work through a proxy [\#950](https://github.com/habitat-sh/habitat/issues/950)
- "build" is not using pkg\_origin, defaulting to "root" [\#940](https://github.com/habitat-sh/habitat/issues/940)
- Studio doesn't work properly with "vi" [\#939](https://github.com/habitat-sh/habitat/issues/939)
- Stream did not contain valid utf-8 [\#925](https://github.com/habitat-sh/habitat/issues/925)
- support core/pip for python [\#904](https://github.com/habitat-sh/habitat/issues/904)
- hab start core/hab-depot with set permissions failed [\#900](https://github.com/habitat-sh/habitat/issues/900)
- Why Rust? [\#891](https://github.com/habitat-sh/habitat/issues/891)
- hab version outside studio doesn't match version inside studio [\#889](https://github.com/habitat-sh/habitat/issues/889)
- Format plans so most frequently changed variables are at the top [\#882](https://github.com/habitat-sh/habitat/issues/882)
- Hab has issues installing package dependencies on Mac [\#879](https://github.com/habitat-sh/habitat/issues/879)
- ./configure: No such file or directory [\#878](https://github.com/habitat-sh/habitat/issues/878)
- docker issue during hab export [\#863](https://github.com/habitat-sh/habitat/issues/863)
- User can be invited to an origin multiple times [\#835](https://github.com/habitat-sh/habitat/issues/835)

## [0.7.0](https://github.com/habitat-sh/habitat/tree/0.7.0) (06-14-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.6.0...0.7.0)

## Features & Enhancements

- bash completion for `hab` [\#602](https://github.com/habitat-sh/habitat/issues/602)

## Bug fixes

- hab studio:  unzip: command not found [\#591](https://github.com/habitat-sh/habitat/issues/591)

**Closed issues:**

- Specify dependency versions [\#848](https://github.com/habitat-sh/habitat/issues/848)
- "No such file or directory" trying to enter the studio [\#736](https://github.com/habitat-sh/habitat/issues/736)

## [0.6.0](https://github.com/habitat-sh/habitat/tree/0.6.0) (05-29-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.5.0...0.6.0)

**Closed issues:**

- Where do you get the core key? [\#582](https://github.com/habitat-sh/habitat/issues/582)

## [0.5.0](https://github.com/habitat-sh/habitat/tree/0.5.0) (05-12-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/jcd/pre-hab-rename...0.5.0)

**Closed issues:**

- The "Create your origin key" in the tutorial assumes 'hab' command present [\#486](https://github.com/habitat-sh/habitat/issues/486)

## [jcd/pre-hab-rename](https://github.com/habitat-sh/habitat/tree/jcd/pre-hab-rename) (03-29-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.4.0...jcd/pre-hab-rename)

## [0.4.0](https://github.com/habitat-sh/habitat/tree/0.4.0) (02-02-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/working_etcd_demo...0.4.0)

## [working_etcd_demo](https://github.com/habitat-sh/habitat/tree/working_etcd_demo) (01-28-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.3.0...working_etcd_demo)

## [0.3.0](https://github.com/habitat-sh/habitat/tree/0.3.0) (01-15-2016)
[Full Changelog](https://github.com/habitat-sh/habitat/compare/0.2.0...0.3.0)

## [0.2.0](https://github.com/habitat-sh/habitat/tree/0.2.0) (11-20-2015)


\* *This Change Log was automatically generated by [github_changelog_generator](https://github.com/skywinder/Github-Changelog-Generator)*