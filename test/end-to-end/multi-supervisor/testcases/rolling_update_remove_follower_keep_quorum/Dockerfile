FROM habitat_integration_base

# Always accept the license when we run this image.
ENV HAB_LICENSE=accept-no-persist

COPY run_test.sh /run_test.sh
RUN chmod a+x /run_test.sh
CMD /run_test.sh
