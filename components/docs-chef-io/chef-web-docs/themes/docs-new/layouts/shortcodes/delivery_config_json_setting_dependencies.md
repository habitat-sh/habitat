The `dependencies` setting specifies run-time dependencies on which the
current project depends. These dependency associations affect how
projects are promoted through the Union, Rehearsal, and Delivered
stages. Dependencies may be defined in the following ways:

-   `"project_name"`
-   `"project_name:pipeline_name"`
-   `"org_name/project_name"`
-   `"org_name/project_name:pipeline_name"`

If only a project name is provided, the master pipeline for that project
is the dependency.