---
title: Supervisor Log Keys
---

# Habitat Supervisor Log Key Reference

When running services with the Habitat Supervisor you'll see log output similar
to this:

~~~
redis.default(SR): Initializing
redis.default(SV): Starting
redis.default(O):
~~~

The letters in parentheses are called "log keys" and signify what type of log
message is being shown. This can be useful for log filtering.

They are mostly organized by the part of the Habitat Supervisor code base that
they are running from, so they often are of value to somebody troubleshooting
problems in the Habitat Supervisor source code.

The meanings of the keys are as follows:

| Key | Description |
|-----|-------------|
| CE | Census |
| CFG | Global configuration |
| CS | Create service: When a service is being started |
| ER| Errors |
| HG | Messages from the HTTP gateway |
| MN | Main |
| MR | Manager |
| O | Standard output |
| PH | Package hooks |
| PK | Package |
| PT | Path |
| SC | Service configuration |
| SH | Starting a shell with `hab sup sh` |
| SI | Unix signals |
| SOT | Structured output |
| SR | Service runtime |
| SU | Service updater |
| SV | Supervisor |
| SY | "sys" utility |
| UR | Users utility |
| UT | Utilities |
