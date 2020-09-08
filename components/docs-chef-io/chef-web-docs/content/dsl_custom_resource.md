+++
title = "Custom Resource DSL"
draft = false

aliases = ["/dsl_custom_resource.html"]

[menu]
  [menu.infra]
    title = "Custom Resources DSL"
    identifier = "chef_infra/cookbook_reference/dsl_custom_resource.md Custom Resources DSL"
    parent = "chef_infra/cookbook_reference"
    weight = 70
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/dsl_custom_resource.md)

Use the Custom Resource DSL to define behaviors within custom resources, such as:

-   Loading the value of a specific property
-   Comparing the current property value against a desired property value
-   Telling Chef Infra Client when and how to make changes

## action_class

{{% dsl_custom_resource_method_converge_if_changed_multiple %}}

## converge_if_changed

{{% dsl_custom_resource_method_converge_if_changed %}}

### Converging Multiple Properties

{{% dsl_custom_resource_method_converge_if_changed_multiple %}}

## default_action

{{% dsl_custom_resource_method_default_action %}}

## load_current_value

{{% dsl_custom_resource_method_load_current_value %}}

### Block Arguments

{{% dsl_custom_resource_method_property_block_argument %}}

## property

{{% dsl_custom_resource_method_property %}}

### ruby_type

{{% dsl_custom_resource_method_property_ruby_type %}}

### validators

{{% dsl_custom_resource_method_property_validation_parameter %}}

### desired_state

{{% dsl_custom_resource_method_property_desired_state %}}

### identity

{{% dsl_custom_resource_method_property_identity %}}

### Working With Properties

The Custom Resource DSL includes several helper methods for accessing and manipulating the values of properties defined within a custom resource.

#### new_resource.property

{{< readFile_shortcode file="dsl_custom_resource_method_new_resource.md" >}}

#### property_is_set?

{{% dsl_custom_resource_method_property_is_set %}}

#### reset_property

{{% dsl_custom_resource_method_reset_property %}}

## provides

{{% dsl_custom_resource_method_provides %}}

## resource_name

{{< note >}}

{{% ruby_style_patterns_hyphens %}}

{{< /note >}}

{{% dsl_custom_resource_method_resource_name %}}

{{% dsl_custom_resource_method_resource_name_example %}}

## deprecated

{{% dsl_custom_resource_method_deprecated %}}
