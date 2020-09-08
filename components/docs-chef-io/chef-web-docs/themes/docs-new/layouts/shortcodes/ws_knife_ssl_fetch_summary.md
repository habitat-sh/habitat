Use the `knife ssl fetch` subcommand to copy SSL certificates from an
HTTPS server to the `trusted_certs_dir` directory that is used by knife
and Chef Infra Client to store trusted SSL certificates. When these
certificates match the hostname of the remote server, running
`knife ssl fetch` is the only step required to verify a remote server
that is accessed by either knife or Chef Infra Client.

<div class="admonition-warning">

<p class="admonition-warning-title">Warning</p>

<div class="admonition-warning-text">

It is the user's responsibility to verify the authenticity of every SSL
certificate before downloading it to the `/.chef/trusted_certs`
directory. knife will use any certificate in that directory as if it is
a 100% trusted and authentic SSL certificate. knife will not be able to
determine if any certificate in this directory has been tampered with,
is forged, malicious, or otherwise harmful. Therefore it is essential
that users take the proper steps before downloading certificates into
this directory.



</div>

</div>