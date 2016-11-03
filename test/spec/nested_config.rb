# it's safest to use `require_relative` with `spec_helper`, 
# as different modes of running the tests have different require 
# behaviors. We can't fully rely on a `.rspec` file.
require_relative 'spec_helper'

describe "Nested Config" do
    before(:all) do
        ctx.common_setup()
    end

    after(:all) do
        ctx.common_teardown()
    end

     # This method lets us prevent cleanup of the test environment
     # in the event of a test failure.
    # You don't need this if you want the test environment to 
    # cleanup all generated test directories and keys even
    # if your tests fail.
    after(:each) do |example|
        if example.exception
            puts "Detected failed examples, keeping environment"
            ctx.cleanup = false
        end
    end

    context "package functionality" do
        it "should build, install, start and render configs for service_with_nested_config" do
            ctx.register_dir "result"

            # Build the package
            result = ctx.cmd_expect("studio build fixtures/service_with_nested_config",
                                    "I love it when a plan.sh comes together",
                                    :timeout_seconds => 60)
            expect(result.exited?).to be true
            expect(result.exitstatus).to eq 0

            last_build_info = HabTesting::Utils::parse_last_build()

            expect(last_build_info["pkg_origin"]).to eq ctx.hab_origin

            hart = Pathname.new("results").join(last_build_info["pkg_artifact"])
            expect(File.exist?(hart)).to be true

            # install package
            result = ctx.cmd_expect("pkg install ./results/#{last_build_info["pkg_artifact"]}",
                                    "Install of #{ctx.hab_origin}/service_with_nested_config/0.0.1/#{last_build_info["pkg_release"]} complete",
                                    :kill_when_found => false)
            
            # verify install completed
            expect(result.exited?).to be true
            expect(result.exitstatus).to eq 0

            install_path = Pathname.new(ctx.hab_pkg_path).join(
                ctx.hab_origin,
                last_build_info["pkg_name"],
                last_build_info["pkg_version"],
                last_build_info["pkg_release"]
            )

            # check installed config files
            expect(File.exist?("#{install_path}/config/base_level.conf")).to be true
            expect(File.exist?("#{install_path}/config/subdir")).to be true
            expect(File.exist?("#{install_path}/config/subdir/one_deep.conf")).to be true
            expect(File.exist?("#{install_path}/config/subdir/nextdir")).to be true
            expect(File.exist?("#{install_path}/config/subdir/nextdir/two_deep.conf")).to be true

            # register output directory
            svc_path = Pathname.new(ctx.hab_svc_path).join("service_with_nested_config")
            ctx.register_dir svc_path

            # switch to sup binary
            ctx.hab_bin = ctx.hab_sup_bin
            # start the service
            result = ctx.cmd_expect("start #{ctx.hab_origin}/service_with_nested_config",
                                    "Starting service_with_nested_config",
                                    :kill_when_found => true)

            # check running config files
            expect(File.exist?(svc_path)).to be true
            expect(File.exist?("#{svc_path}/config/base_level.conf")).to be true
            expect(File.exist?("#{svc_path}/config/subdir")).to be true
            expect(File.exist?("#{svc_path}/config/subdir/one_deep.conf")).to be true
            expect(File.exist?("#{svc_path}/config/subdir/nextdir")).to be true
            expect(File.exist?("#{svc_path}/config/subdir/nextdir/two_deep.conf")).to be true
        end
    end
end