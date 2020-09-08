A Policyfile file allows you to specify in a single document the
cookbook revisions and recipes that Chef Infra Client will apply. A
Policyfile file is uploaded to the Chef Infra Server, where it is
associated with a group of nodes. When these nodes are configured during
a Chef Infra Client run, Chef Infra Client will make decisions based on
your Policyfile settings and will build a run-list based on that
information. A Policyfile file may be versioned, and then promoted
through deployment stages to safely and reliably deploy new
configuration.