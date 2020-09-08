The `skip_phases` setting specifies which phases are skipped by Chef
Automate during the execution of a change through the pipeline. If a
phase is defined as skipped, this applies to all stages in the pipeline.

Currently, the `functional.rb`, `quality.rb`, `security.rb`, and
`smoke.rb` recipes are blank by default and should be set to skipped in
the `config.json` file:

``` javascript
"skip_phases": [
  "functional",
  "quality",
  "security",
  "smoke"
]
```