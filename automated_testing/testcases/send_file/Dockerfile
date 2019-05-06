FROM habitat_integration_base

# Always accept the license when we run this image.
ENV HAB_LICENSE=accept-no-persist

# We're going to be using docker inside the container.
#
# TODO: Consider ensuring that the base container itself has Docker if
# this becomes more of a pattern.
RUN hab pkg install core/docker -b -f

COPY run_test.sh /run_test.sh
RUN chmod a+x /run_test.sh
CMD /run_test.sh
