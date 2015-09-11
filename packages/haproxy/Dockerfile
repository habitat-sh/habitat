FROM bldr/base
MAINTAINER Adam Jacob <adam@chef.io>
WORKDIR /
COPY haproxy-cache/ /opt/bldr/pkgs/bldr/haproxy/1.5.12/20150911202527/
COPY glibc-cache/ /opt/bldr/pkgs/bldr/glibc/2.19/20150911201554/
COPY pcre-cache/ /opt/bldr/pkgs/bldr/pcre/8.37/20150911202351/
COPY openssl-cache/ /opt/bldr/pkgs/bldr/openssl/1.0.2d/20150911201701/
COPY zlib-cache/ /opt/bldr/pkgs/bldr/zlib/1.2.8/20150911201555/
VOLUME /opt/bldr/srvc/haproxy/data /opt/bldr/srvc/haproxy/config
EXPOSE 80 443 9631
CMD ["start", "haproxy"]
