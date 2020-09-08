The authentication process ensures the Chef Infra Server responds only
to requests made by trusted users. Public key encryption is used by the
Chef Infra Server. When a node and/or a workstation is configured to run
Chef Infra Client, both public and private keys are created. The public
key is stored on the Chef Infra Server, while the private key is
returned to the user for safe keeping. (The private key is a .pem file
located in the `.chef` directory or in `/etc/chef`.)

Both Chef Infra Client and knife use the Chef Infra Server API when
communicating with the Chef Infra Server. The chef-validator uses the
Chef Infra Server API, but only during the first Chef Infra Client run
on a node.

Each request to the Chef Infra Server from those executables sign a
special group of HTTP headers with the private key. The Chef Infra
Server then uses the public key to verify the headers and verify the
contents.