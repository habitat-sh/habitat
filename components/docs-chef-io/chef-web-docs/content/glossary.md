+++
title = "Glossary"
draft = false

aliases = ["/glossary.html"]

[menu]
  [menu.infra]
    title = "Glossary"
    identifier = "chef_infra/getting_started/glossary.md Glossary"
    parent = "chef_infra/getting_started"
    weight = 70
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/glossary.md)

**Acceptance**

:   A Workflow stage. The Acceptance stage is where your team decides
    whether the submitted change should ship all the way out to its
    final destination.

**analytics rules**

:   Event tracking during a Chef Infra Client run that generates data
    made visible to Workflow.

**Automate**

:   A full suite of enterprise capabilities for maintaining continuous
    visibility into application, infrastructure, and security
    automation.

**Berkshelf**

:   Manage cookbook dependencies.

**Build**

:   A Workflow stage. The purpose of the Build stage is to assemble one
    or more potentially releasable artifacts and make them available to
    the remaining stages of the pipeline. Using Berkshelf can help to
    manage cookbook dependencies.

**chef-apply**

:   A command-line tool that allows a single recipe to be run from the
    command line.

**ChefDK**

:   The legacy package of tools for developing Chef Infra cookbooks.
    This product has been superseded by Chef Workstation which should be
    used instead.

**Chef Infra Client**

:   A command-line tool that that runs Chef. Also, the name of Chef as
    it is installed on a node.

**Chef Workstation**

:   A collection of tools to aide in development of Chef Infra
    cookbooks. It uses the full stack installer to give you everything
    you need to get going in one package. You can download it at [Chef
    Workstation](https://downloads.chef.io/chef-workstation/).

**chef-repo**

:   The repository structure in which cookbooks are authored, tested,
    and maintained. View [an example of
    the](https://github.com/chef/chef-repo) chef-repo.

**Chef Infra Server**

:   The Chef Infra Server acts as a hub for configuration data. The Chef
    Infra Server stores cookbooks, the policies that are applied to
    nodes, and metadata that describes each registered node that is
    being managed by Chef Infra Client. Nodes use Chef Infra Client to
    ask the Chef Infra Server for configuration details, such as
    recipes, templates, and file distributions.

**ChefSpec**

:   ChefSpec is a unit-testing framework for testing Chef Infra
    cookbooks.

**chef-zero**

:   A very lightweight Chef Infra Server that runs in-memory on the
    local machine during a Chef Infra Client run. Also known as local
    mode.

**cookbook**

:   A cookbook is the fundamental unit of configuration and policy
    distribution.

**Cookstyle**

:   A linting tool that helps you write better Chef Infra cookbooks by
    detecting and automatically correcting style, syntax, and logic
    mistakes in your code.

**custom resource**

:   An extension to Chef Infra Client that allows you to ship your own
    reusable resources within a cookbook.

**data bag**

:   A data_bag is a global variable that is stored as JSON data and is
    accessible from a Chef Infra Server.

**definition**

:   A definition is code that is reused across recipes, similar to a
    compile-time macro, and is defined in a cookbook. Definitions are
    not recommended for new users and Custom Resources should be written
    for reusable code instead.

**Delivered**

:   A Workflow stage. Delivered is the final stage of the pipeline, what
    it means for your system is up to you. It could mean deploying the
    change so that it is live and receiving production traffic, or it
    might mean publishing a set of artifacts so they are accessible for
    your customers.

**environment**

:   An environment is a way to map an organization's real-life workflow
    to what can be configured and managed when using Chef Infra Server.

**Foodcritic**

:   A legacy linting tool for doing static code analysis on cookbooks.
    This tool has been replaced with Cookstyle which should be used
    instead.

**knife**

:   A command-line tool that provides an interface between a local
    chef-repo and the Chef Infra Server. Use it to manage nodes,
    cookbooks, recipes, roles, data bags, environments, bootstrapping
    nodes, searching the Chef Infra Server, and more.

**library**

:   A library allows arbitrary Ruby code to be included in a cookbook,
    either as a way of extending the classes that are built-in to Chef
    Infra Client or by implementing entirely new functionality.

**Management Console**

:   The Chef web-based management console you can use to manage Role
    Based Access Control (RBAC), edit and delete nodes, and reset
    private keys. Keep up to date with what's happening during Chef
    Infra Client runs across an entire organization or on specific
    nodes.

**node**

:   A node is any physical, virtual, or cloud device that is configured
    and maintained by an instance of Chef Infra Client.

**node object**

:   A node object is a history of the attributes, run-lists, and roles
    that were used to configure a node that is under management by Chef
    Infra.

**ohai**

:   Ohai is a tool that is used to detect attributes on a node, and then
    provide these attributes to Chef Infra Client at the start of every
    run.

**organization**

:   An organization is a single instance of a Chef Infra Server,
    including all of the nodes that are managed by that Chef Infra
    Server and each of the workstations that will run knife and access
    the Chef Infra Server using the Chef Infra Server API.

**pipeline**

:   A pipeline is series of automated and manual quality gates that take
    software changes from development to delivery. Pipelines in Workflow
    have six stages: Verify, Build, Acceptance, Union, Rehearsal, and
    Delivered.

**policy**

:   Policy settings can be used to map business and operational
    requirements, such as process and workflow, to settings and objects
    stored on the Chef Infra Server. See roles, environments, and data
    bags.

**Push Jobs**

:   Allows you to execute commands across hundreds or even thousands of
    nodes in your Chef-managed infrastructure.

**recipe**

:   A recipe is a collection of resources that tells Chef Infra Client
    how to configure a node.

**Rehearsal**

:   If all phases of Union succeed, then the Rehearsal stage is
    triggered. Rehearsal increases confidence in the artifacts and the
    deployment by repeating the process that occurred in Union in a
    different environment.

**Reporting**

:   Capture and visualize what happens during the execution of Chef
    Infra Client runs across all of your Chef-managed infrastructure.

**resource**

:   A resource is a statement of configuration policy that describes the
    desired state of an piece within your infrastructure, along with the
    steps needed to bring that item to the desired state.

**role**

:   A role is a way to define certain patterns and processes that exist
    across nodes in an organization as belonging to a single job
    function.

**run-list**

:   A run-list defines all of the configuration settings that are
    necessary for a node that is under management by Chef to be put into
    the desired state and the order in which these configuration
    settings are applied.

**Test Kitchen**

:   Test Kitchen is an integration framework that is used to
    automatically test cookbook data across any combination of platforms
    and test suites. Test Kitchen is packaged in Chef Workstation.

**Union**

:   A Workflow stage. Union is the first of the three shared pipeline
    stages. The purpose of the Union stage is to assess the impact of
    the change in the context of a complete (or as close as possible)
    installation of the set of projects that comprise the system as a
    whole.

**Verify**

:   A Workflow stage. The purpose of Verify is to run checks so that the
    system can decide if it's worth the time of a human to review the
    submitted change.

**visibility**

:   A feature of Workflow that provides real-time visibility into what
    is happening on the Chef Infra Server, including what's changing,
    who made those changes, and when they occurred.

**workflow**

:   A feature of Workflow that manages changes to both infrastructure
    and application code, giving your operations and development teams a
    common platform for developing, building, testing, and deploying
    cookbooks, applications, and more. For more information see the
    [Chef Automate Overview](/workflow%20.html).
