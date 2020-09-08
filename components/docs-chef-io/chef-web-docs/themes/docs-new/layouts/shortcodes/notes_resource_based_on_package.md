In many cases, it is better to use the **package** resource instead of
this one. This is because when the **package** resource is used in a
recipe, Chef Infra Client will use details that are collected by Ohai at
the start of a Chef Infra Client run to determine the correct package
application. Using the **package** resource allows a recipe to be
authored in a way that allows it to be used across many platforms.