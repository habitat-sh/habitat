---
name: Bug Report
about: Explain a defect discovered in Habitat
labels: C-bug

---

Hello - thanks for reporting an issue with Habitat.

In order to help us troubleshoot the issue, please be sure to include the following information if applicable:

- [ ] The OS (including version) where you are running any of the Habitat commands.
- [ ] Debug/backtrace of the command you are trying to run. You can set the following environment variables before running the `hab` command to generate a trace:

		# Linux/MacOS
		RUST_LOG=debug RUST_BACKTRACE=1 hab ...
		
		# Windows (Powershell 5+)
		$env:RUST_LOG="debug"; $env:RUST_BACKTRACE=1; hab ...
		
- [ ] Current Habitat environment variables where the `hab` command or supervisor is running.  These can be gathered using:

		env | grep HAB | grep -v HAB_AUTH_TOKEN

	- ***DO NOT include `HAB_AUTH_TOKEN`*** or any other sensitive information. More info on Habitat environment variables can be found [here](https://www.habitat.sh/docs/reference/environment-vars/).
	- [ ] If you're running into problems with the [Habitat Studio](https://www.habitat.sh/docs/concepts-studio/), please use the `env | grep ...` command specified above to include environment variables:
		- [ ] a. before starting the studio
		- [ ] b. inside the studio

- [ ] If this is a key related issue, please include the list of files (including user/group permissions) in `/hab/cache/keys` and `$HOME/.hab/cache/keys` via `ls -la`.

		# Linux/MacOS
		ls -la /hab/cache/keys
		ls -la $HOME/.hab/cache.keys

		# Windows (Powershell 5+)
		ls C:\hab\cache\keys
