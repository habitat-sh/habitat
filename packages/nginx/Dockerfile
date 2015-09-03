FROM bldr/base
MAINTAINER Adam Jacob <adam@chef.io>
WORKDIR /
COPY nginx-cache/ /opt/bldr/pkgs/bldr/nginx/1.8.0/20150831221630/
COPY glibc-cache/ /opt/bldr/pkgs/bldr/glibc/2.19/20150831220956/
COPY libedit-cache/ /opt/bldr/pkgs/bldr/libedit/3.1.20150325/20150831221558/
COPY ncurses-cache/ /opt/bldr/pkgs/bldr/ncurses/5.9/20150831221532/
COPY zlib-cache/ /opt/bldr/pkgs/bldr/zlib/1.2.8/20150831220957/
COPY bzip2-cache/ /opt/bldr/pkgs/bldr/bzip2/1.0.6/20150831221606/
COPY openssl-cache/ /opt/bldr/pkgs/bldr/openssl/1.0.2d/20150831221048/
COPY pcre-cache/ /opt/bldr/pkgs/bldr/pcre/8.37/20150831221608/
VOLUME /opt/bldr/srvc/nginx/data /opt/bldr/srvc/nginx/config
EXPOSE 80 443 9631
CMD ["start", "nginx"]
