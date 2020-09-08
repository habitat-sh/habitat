Windows will throw errors when path name lengths are too long. For this
reason, it's often helpful to use a very short top-level directory, much
like what is done in UNIX and Linux. For example, Chef uses `/opt/` to
install Chef Workstation on macOS. A similar approach can be done on
Microsoft Windows, by creating a top-level directory with a short name.
For example: `C:\chef`.