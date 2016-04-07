# bldr plans

This directory contains the plans for building software with bldr.

The Makefile has a target to create a new plan. Invoke it with:

```bash
make new-plan plan=zombocom
```

This will create a `plans/zombocom` directory and copy the the `plan-tmpl.sh` to `plans/zombocom/plan.sh`.
