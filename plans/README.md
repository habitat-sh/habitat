# Habitat Plans

This directory contains Plans that build software for the Habitat ecosystem.

The Makefile has a target to create a new plan. Invoke it with:

```bash
make new-plan plan=zombocom
```

This will create a `plans/zombocom` directory and copy the the `plan-tmpl.sh` to `plans/zombocom/plan.sh`.
