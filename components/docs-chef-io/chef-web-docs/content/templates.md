+++
title = "About Templates"
draft = false

aliases = ["/templates.html"]

[menu]
  [menu.infra]
    title = "Templates"
    identifier = "chef_infra/cookbook_reference/templates.md Templates"
    parent = "chef_infra/cookbook_reference"
    weight = 90
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/templates.md)

{{% template %}}

{{< note >}}

{{% notes_cookbook_template_erubis %}}

{{< /note >}}

## Requirements

{{< readFile_shortcode file="template_requirements.md" >}}

## Variables

{{< readFile_shortcode file="template_variables.md" >}}

## File Specificity

{{% template_specificity %}}

{{% template_specificity_pattern %}}

{{% template_specificity_example %}}

## Host Notation

{{% template_host_notation %}}

## Transfer Frequency

{{% template_transfer_frequency %}}

## Partial Templates

{{% template_partials %}}

### variables Attribute

{{< readFile_shortcode file="template_partials_variables_attribute.md" >}}

### render Method

{{< readFile_shortcode file = "template_partials_render_method.md" >}}
