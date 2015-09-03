FROM bldr/base
MAINTAINER Adam Jacob <adam@chef.io>
WORKDIR /
COPY haproxy-cache/ /opt/bldr/pkgs/bldr/haproxy/1.5.12/20150825224349/
COPY glibc-cache/ /opt/bldr/pkgs/bldr/glibc/2.19/20150825223126/
COPY pcre-cache/ /opt/bldr/pkgs/bldr/pcre/8.37/20150825224112/
COPY openssl-cache/ /opt/bldr/pkgs/bldr/openssl/1.0.2a/20150825223258/
COPY zlib-cache/ /opt/bldr/pkgs/bldr/zlib/1.2.8/20150825223128/
VOLUME /opt/bldr/srvc/haproxy/data /opt/bldr/srvc/haproxy/config
EXPOSE 80 443 9631
CMD ["start", "haproxy"]
