---
name: "\U0001F41B Bug report"
about: "Something isn't working as expected \U0001F914. "

---

### `hab` CLI version
[Version of the habitat binary where you are encountering the issue]

### Platform Details
[Operating system distribution and release version. Cloud provider if running in the cloud]

### Scenario:
[What you are trying to achieve and you can't?]

### Steps to Reproduce:
[If you are filing an issue what are the things we need to do in order to repro your problem? How are you using the CLI?]
- If this is a key related issue, please include the list of files (including user/group permissions) in `/hab/cache/keys` and `$HOME/.hab/cache/keys` via `ls -la`.

### Environment Configuration
[Current Habitat environment variables where the `hab` command or supervisor is running. If you're running into problems with the [Habitat Studio](https://www.habitat.sh/docs/concepts-studio/), please provide environment variables both BEFORE starting the studio and INSIDE the studio]  
- Environment Variables can be gathered using:
```
		env | grep HAB | grep -v HAB_AUTH_TOKEN
```

- ***DO NOT include `HAB_AUTH_TOKEN`*** or any other sensitive information. More info on Habitat environment variables can be found [here](https://www.habitat.sh/docs/reference/environment-vars/).

### Backtrace/Debug
[Please provide a Debug/backtrace of the command you are trying to run.]
- You can set the following environment variables before running the `hab` command to generate a trace:
```
		# Linux/MacOS
		RUST_LOG=debug RUST_BACKTRACE=1 hab ...
		
		# Windows (Powershell 5+)
		$env:RUST_LOG="debug"; $env:RUST_BACKTRACE=1; hab ...
```
### Expected Result:
[What are you expecting to happen as the consequence of above reproduction steps?]

### Actual Result:
[What actually happens after the reproduction steps? Include the error output or a link to a gist if possible.]
