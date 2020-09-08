Use the `edit` argument to edit the data contained in a data bag. If
encryption is being used, the data bag will be decrypted, the data will
be made available in the \$EDITOR, and then encrypted again before
saving it to the Chef Infra Server.