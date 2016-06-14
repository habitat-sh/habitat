We don't generate or provide any keys or certificates for OpenVPN. You'll need to generate your own and populate the server. You can do that with Habitat using the `file upload` command.

On the system that is to run OpenVPN:

```
hab service key generate openvpn.default ORGNAME
hab start core/openvpn --group default --org ORGNAME
```

On the local system that has the OpenVPN keys/certificates (e.g., a laptop/workstation), copy the generated `openvpn.default@ORGNAME-timestamp.pub` file (e.g., in `/hab/cache/keys`) to the local system to the key cache (e.g., `~/.hab/cache/keys`). Then do the following:

```
hab user key generate USERNAME
```

Copy the generated `USERNAME-timestamp.pub` to the OpenVPN system key cache (e.g., `/hab/cache/keys`). Then upload the OpenVPN certificates and keys.

```
for i in ca.crt server.crt server.key dh1024.pem
do
  hab file upload --org ORGNAME --peer 172.17.0.4 openvpn.default $i 1 USERNAME
done
```
