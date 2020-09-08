A cookbook version represents a set of functionality that is different
from the cookbook on which it is based. A version may exist for many
reasons, such as ensuring the correct use of a third-party component,
updating a bug fix, or adding an improvement. A cookbook version is
defined using syntax and operators, may be associated with environments,
cookbook metadata, and/or run-lists, and may be frozen (to prevent
unwanted updates from being made).

A cookbook version is maintained just like a cookbook, with regard to
source control, uploading it to the Chef Infra Server, and how Chef
Infra Client applies that cookbook when configuring nodes.