Chef Infra Client processes recipes in two phases:

1.  First, each resource in the node object is identified and a resource
    collection is built. All recipes are loaded in a specific order, and
    then the actions specified within each of them are identified. This
    is also referred to as the "compile phase".
2.  Next, Chef Infra Client configures the system based on the order of
    the resources in the resource collection. Each resource then
    examines the node and performs the necessary steps to complete the
    action. This is also referred to as the "execution phase".

Typically, actions are processed during the execution phase of a Chef
Infra Client run. However, sometimes it is necessary to run an action
during the compile phase. For example, a resource can be configured to
install a package during the compile phase to ensure that application is
available to other resources during the execution phase.

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

Use the **chef_gem** resource to install gems that are needed by Chef
Infra Client during the execution phase.



</div>

</div>