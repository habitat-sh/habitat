The following example shows a run-time dependency against the master
branch of a project named `BackendAPI`:

``` javascript
{
  "version": "2",
  "build_cookbook": {
    "name": "build-cookbook",
    "path": ".delivery/build-cookbook"
  },
  "skip_phases": [],
  "dependencies": ["BackendAPI"]
}
```