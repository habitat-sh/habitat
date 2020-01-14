FROM habitat_integration_base

# Always accept the license when we run this image.
ENV HAB_LICENSE=accept-no-persist

COPY run_test.ps1 /run_test.ps1
CMD pwsh /scripts/end_to_end/run_e2e_test_core.ps1 /run_test.ps1
