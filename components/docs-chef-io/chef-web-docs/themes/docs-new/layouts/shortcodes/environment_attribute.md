Attributes can be defined in an environment and then used to override
the default attributes in a cookbook. When an environment is applied
during a Chef Infra Client run, environment attributes are compared to
the attributes that are already present on the node. When the
environment attributes take precedence over the default attributes, Chef
Infra Client applies those new settings and values during a Chef Infra
Client run.

Environment attributes can be set to either `default` attribute level or
an `override` attribute level.