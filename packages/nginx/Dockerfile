FROM bldr/base
MAINTAINER Adam Jacob <adam@chef.io>
WORKDIR /
COPY nginx-cache/ /opt/bldr/pkgs/bldr/nginx/1.8.0/20150917181355/
COPY glibc-cache/ /opt/bldr/pkgs/bldr/glibc/2.19/20150917180623/
COPY libedit-cache/ /opt/bldr/pkgs/bldr/libedit/3.1.20150325/20150917181319/
COPY ncurses-cache/ /opt/bldr/pkgs/bldr/ncurses/5.9/20150917181251/
COPY zlib-cache/ /opt/bldr/pkgs/bldr/zlib/1.2.8/20150917180624/
COPY bzip2-cache/ /opt/bldr/pkgs/bldr/bzip2/1.0.6/20150917181328/
COPY openssl-cache/ /opt/bldr/pkgs/bldr/openssl/1.0.2d/20150917180717/
COPY pcre-cache/ /opt/bldr/pkgs/bldr/pcre/8.37/20150917181331/
VOLUME /opt/bldr/srvc/nginx/data /opt/bldr/srvc/nginx/config
EXPOSE 80 443 9631
CMD ["start", "nginx"]
