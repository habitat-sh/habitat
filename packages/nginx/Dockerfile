FROM bldr/base
MAINTAINER Adam Jacob <adam@chef.io>
WORKDIR /
COPY nginx-cache/ /opt/bldr/pkgs/bldr/nginx/1.8.0/20150825224152/
COPY glibc-cache/ /opt/bldr/pkgs/bldr/glibc/2.19/20150825223126/
COPY libedit-cache/ /opt/bldr/pkgs/bldr/libedit/3.1.20150325/20150825224052/
COPY ncurses-cache/ /opt/bldr/pkgs/bldr/ncurses/5.9/20150825224002/
COPY zlib-cache/ /opt/bldr/pkgs/bldr/zlib/1.2.8/20150825223128/
COPY bzip2-cache/ /opt/bldr/pkgs/bldr/bzip2/1.0.6/20150825224108/
COPY openssl-cache/ /opt/bldr/pkgs/bldr/openssl/1.0.2a/20150825223258/
COPY pcre-cache/ /opt/bldr/pkgs/bldr/pcre/8.37/20150825224112/
VOLUME /opt/bldr/srvc/nginx/data /opt/bldr/srvc/nginx/config
EXPOSE 80 443 9631
CMD ["start", "nginx"]
