FROM bldr/base
MAINTAINER Adam Jacob <adam@chef.io>
WORKDIR /
COPY nginx-cache/ /opt/bldr/pkgs/bldr/nginx/1.8.0/20150911202425/
COPY glibc-cache/ /opt/bldr/pkgs/bldr/glibc/2.19/20150911201554/
COPY libedit-cache/ /opt/bldr/pkgs/bldr/libedit/3.1.20150325/20150911202331/
COPY ncurses-cache/ /opt/bldr/pkgs/bldr/ncurses/5.9/20150911202253/
COPY zlib-cache/ /opt/bldr/pkgs/bldr/zlib/1.2.8/20150911201555/
COPY bzip2-cache/ /opt/bldr/pkgs/bldr/bzip2/1.0.6/20150911202343/
COPY openssl-cache/ /opt/bldr/pkgs/bldr/openssl/1.0.2d/20150911201701/
COPY pcre-cache/ /opt/bldr/pkgs/bldr/pcre/8.37/20150911202351/
VOLUME /opt/bldr/srvc/nginx/data /opt/bldr/srvc/nginx/config
EXPOSE 80 443 9631
CMD ["start", "nginx"]
