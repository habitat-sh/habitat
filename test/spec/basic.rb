# Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

require 'fileutils'
require 'pathname'
require 'tempfile'
require 'timeout'

# TODO: .rspec file doesn't seem to be honored, so we need
# to manually include the spec_helper here
require_relative 'spec_helper'

describe "Habitat CLI" do
    # TODO: maybe extract this into a module in the future
    before(:all) do
        ctx.common_setup()
        #to see all command output:
        #ctx.cmd_debug = true
    end

    after(:all) do
        ctx.common_teardown()
    end

    after(:each) do |example|
        if example.exception
            puts "Detected failed examples, not cleaning environment"
            ctx.cleanup = false
        end
    end

    # these are in RSpec instead of Inspec because we
    # keep some ctx independent paths inside the ctx.
    # Perhaps this could be shared in the future.
    context "core cli binaries" do
        it "hab command should be compiled" do
            expect(File.exist?(ctx.hab_bin)).to be true
            expect(File.executable?(ctx.hab_bin)).to be true
        end

        it "hab-sup command should be compiled" do
            expect(File.exist?(ctx.hab_sup_bin)).to be true
            expect(File.executable?(ctx.hab_sup_bin)).to be true
        end
    end

    context "package functionality" do
        # this is example is somewhat larger than desired, however, it exercises
        # build/install/start with a simple package, instead of downloading a
        # prebuilt one from the Depot
        it "should build, install and start a simple service without failure" do
            # we're using `hab studio build`, which generates a ./results
            # directory. We register this directory with the test framework
            # and if the tests pass, the directory will be cleaned up
            # upon success
            ctx.register_dir "results"

            # building a package can take quite awhile, let's bump the timeout to
            # 60 seconds to be sure we finish in time.
            result = ctx.hab_cmd_expect("studio build fixtures/simple_service",
                                        "I love it when a plan.sh comes together",
                                        :timeout_seconds => 300, :debug => false)
            # as the build command MUST complete, we check return code
            expect(result.exited?).to be true
            expect(result.exitstatus).to eq 0

            # read the ./results/last_build.env into a Hash
            last_build = HabTesting::Utils::parse_last_build()
            puts last_build if ctx.cmd_debug

            # did hab-plan-build generate the correct values
            expect(last_build["pkg_origin"]).to eq ctx.hab_origin
            expect(last_build["pkg_name"]).to eq "simple_service"
            expect(last_build["pkg_version"]).to eq "0.0.1"

            # look for the generated .hart file
            built_artifact = Pathname.new("results").join(last_build["pkg_artifact"])
            expect(File.exist?(built_artifact)).to be true

            # register the output directory so files will be cleaned up if tests pass
            ctx.register_dir "#{ctx.hab_pkg_path}/#{ctx.hab_origin}"

            # install the package
            result = ctx.hab_cmd_expect("pkg install ./results/#{last_build["pkg_artifact"]}",
                                        "Install of #{ctx.hab_origin}/simple_service/0.0.1/"\
                                        "#{last_build["pkg_release"]} complete",
                                        :kill_when_found => false)
            # as the installation command MUST complete, we check return code
            expect(result.exited?).to be true
            expect(result.exitstatus).to eq 0

            # use the last_build.env values to generate the installed path
            installed_path = Pathname.new(ctx.hab_pkg_path).join(
                ctx.hab_origin,
                last_build["pkg_name"],
                last_build["pkg_version"],
                last_build["pkg_release"])

            # did the install actually create the path
            expect(File.exist?(installed_path)).to be true

            pkg_files = %w(FILES IDENT MANIFEST PATH TARGET bin default.toml)
            pkg_files.each do |mf|
                expect(File.exist?(Pathname.new(installed_path).join(mf))).to be true
            end

            # register the output directory so files will be cleaned up if tests pass
            ctx.register_dir Pathname.new(ctx.hab_svc_path).join("simple_service")

            # this should start relatively quickly, so we'll use the default timeout
            # This is a long running process, so kill it when we've found the output
            # that we're looking for.
            result = ctx.sup_cmd_expect("start #{ctx.hab_origin}/simple_service",
                                        "Shipping out to Boston",
                                        :kill_when_found => true)
            # don't check the process status here, we killed it!
        end
    end


    # this test installs a previous version of hab-sup that created
    # "run" and "static" symlinks in /hab/svc/foo/. The latest hab-sup
    # copies run and static instead, but we shouldn't crash if we
    # detect an existing run or static symlink.
    it "should not fail with older Habitat run and static symlinks" do
        ctx.register_dir "results"

        # install an older version of hab-sup that creates
        # static and run symlinks
        result = ctx.hab_cmd_expect("pkg install core/hab-sup/0.6.0/20160612142506",
                                    "Install of core/hab-sup/0.6.0/20160612142506 complete",
                                    :timeout_seconds => 60)
        expect(result.exited?).to be true
        expect(result.exitstatus).to eq 0

        # build a fixture
        result = ctx.hab_cmd_expect("studio build fixtures/simple_service_with_run_and_static",
                                    "I love it when a plan.sh comes together",
                                    :timeout_seconds => 60, :debug => false)

        expect(result.exited?).to be true
        expect(result.exitstatus).to eq 0

        # read the ./results/last_build.env into a Hash
        last_build = HabTesting::Utils::parse_last_build()

        # register the output directory so files will be cleaned up if tests pass
        ctx.register_dir "#{ctx.hab_pkg_path}/#{ctx.hab_origin}"
        # cleanup the service directory when finished
        ctx.register_dir "#{ctx.hab_svc_path}/simple_service_with_run_and_static"

        # install the fixture
        result = ctx.hab_cmd_expect("pkg install ./results/#{last_build["pkg_artifact"]}",
                                    "Install of #{ctx.hab_origin}/simple_service_with_run_and_static/0.0.1/"\
                                    "#{last_build["pkg_release"]} complete",
                                    :kill_when_found => false,
                                    :debug => false)
        # as the installation command MUST complete, we check return code
        expect(result.exited?).to be true
        expect(result.exitstatus).to eq 0


        # start the fixture and ensure we have static and run symlinks
        old_hab_sup_path = Pathname.new(ctx.hab_pkg_path).join("core",
                                                               "hab-sup",
                                                               "0.6.0",
                                                               "20160612142506",
                                                               "bin",
                                                               "hab-sup")


        static_src  = Pathname.new(ctx.hab_pkg_path).join(ctx.hab_origin,
                                                          "simple_service_with_run_and_static",
                                                          "0.0.1",
                                                          last_build["pkg_release"],
                                                          "static")
        static_dest = Pathname.new(ctx.hab_svc_path).join("simple_service_with_run_and_static", "static")

        FileUtils.mkdir_p Pathname.new(ctx.hab_svc_path).join("simple_service_with_run_and_static")
        File.symlink(static_src, static_dest)

        result = ctx.cmd_expect("start #{ctx.hab_origin}/simple_service_with_run_and_static",
                                "Shipping out to Boston",
                                :timeout_seconds => 60,
                                :kill_when_found => true,
                                :bin => old_hab_sup_path,
                                :debug => false)


        run_path = Pathname.new(ctx.hab_svc_path).join("simple_service_with_run_and_static", "run")
        expect(File.exist?(run_path)).to be true
        # it's a symlink!
        expect(File.symlink?(run_path)).to be true

        expect(File.exist?(static_dest)).to be true
        # it's a symlink!
        expect(File.symlink?(static_dest)).to be true

        # remove the previous version of hab and use the latest
        # Probably not needed, but we need to be careful which hab-sup binary we're calling to
        # via cli calls to `hab`
        old_pkg_path = Pathname.new(ctx.hab_pkg_path).join("core",
                                                           "hab-sup",
                                                           "0.6.0",
                                                           "20160612142506")
        FileUtils.rm_rf(old_pkg_path);

        # start with the LASTEST version of hab-sup, which should remove the symlink
        # and copy the run file
        result = ctx.sup_cmd_expect("start #{ctx.hab_origin}/simple_service_with_run_and_static",
                                    "Shipping out to Boston",
                                    :timeout_seconds => 60,
                                    :kill_when_found => true,
                                    :debug => false)

        expect(File.exist?(run_path)).to be true
        # ensure that "run" isn't a symlink anymore
        expect(File.symlink?(run_path)).to be false

        expect(File.exist?(static_dest)).to be true
        # ensure that "static" isn't a symlink anymore
        expect(File.symlink?(static_dest)).to be false
    end


    def start_and_config_apply(package, expected_reconfig_text)
        ctx.register_dir "results"

        # build a fixture
        result = ctx.hab_cmd_expect("studio build fixtures/#{package}",
                                    "I love it when a plan.sh comes together",
                                    :timeout_seconds => 60, :debug => false)

        expect(result.exited?).to be true
        expect(result.exitstatus).to eq 0

        # read the ./results/last_build.env into a Hash
        last_build = HabTesting::Utils::parse_last_build()

        # register the output directory so files will be cleaned up if tests pass
        ctx.register_dir "#{ctx.hab_pkg_path}/#{ctx.hab_origin}"
        # cleanup the service directory when finished
        ctx.register_dir "#{ctx.hab_svc_path}/#{package}"

        # install the fixture
        result = ctx.hab_cmd_expect("pkg install ./results/#{last_build["pkg_artifact"]}",
                                    "Install of #{ctx.hab_origin}/#{package}/0.0.1/"\
                                    "#{last_build["pkg_release"]} complete",
                                    :kill_when_found => false,
                                    :debug => false)
        # as the installation command MUST complete, we check return code
        expect(result.exited?).to be true
        expect(result.exitstatus).to eq 0

        tomlfile = Tempfile.new('new_config')
        tomlfile.write("foo = 1")
        timeout = 30


        Timeout::timeout(timeout) do
            ctx.sup_spawn("start #{ctx.hab_origin}/#{package}") do |stdin, stdout_err, t|
                # wait for the service to fully start
                loop do
                    line = stdout_err.readline()
                    puts line
                    if line.include?("Shipping out to Boston") then
                        break
                    end
                end

                # upload a new config to trigger a restart
                ctx.cmd("config apply -p #{ctx.local_ip}:9638 #{package}.default 1 #{tomlfile.path}")
                loop do
                    line = stdout_err.readline()
                    puts line
                    if line.include?(expected_reconfig_text) then
                        break
                    end
                end
            end # ctx.sup_spawn
        end # Timeout::timeout
    end

    it "doesn't send a SIGTERM to child process if reconfigure hook exists" do
        start_and_config_apply("simple_service_with_reconfigure", "RUNNING RECONFIGURE HOOK")
    end

    it "sends a SIGTERM to a child process if a reconfigure hook doesn't exist" do
        start_and_config_apply("simple_service_without_reconfigure", "RECEIVED TERM")
    end
end
