A Policyfile is an optional way to manage role, environment, and
community cookbook data with a single document that is uploaded to the
Chef Infra Server. The file is associated with a group of nodes,
cookbooks, and settings. When these nodes perform a Chef Infra Client
run, they utilize recipes specified in the Policyfile run-list.