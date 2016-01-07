FROM chroot_build:squash
MAINTAINER The Bldr Maintainers <bldr@chef.io>

ENV CHROOT /mnt/chroot
ENV PLANS /plans
ENV PATH $PATH:$PLANS/support/chroot/scripts

WORKDIR /plans/support/chroot
CMD ["make", "chroot-shell"]
