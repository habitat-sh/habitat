FROM bldr_base
MAINTAINER Adam Jacob <adam@chef.io>
WORKDIR /
RUN bldr key -u http://192.168.59.104:8080/keys/chef-public.asc
RUN bldr install redis -u http://192.168.59.104:8080/pkgs/bldr-redis-3.0.1-20150526202848.bldr
VOLUME /opt/bldr/srvc/redis/data
EXPOSE 6379
CMD ["bldr", "start", "redis"]
