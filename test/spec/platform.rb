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
require 'ipaddr'
require 'mixlib/shellout'
require 'open3'
require 'pathname'
require 'securerandom'
require 'shellwords'
require 'singleton'
require 'time'
require 'timeout'


# TODO: move these?
require 'json'
require 'net/http'
require "uri"

module HabTesting


    module Constants
        Metafiles = %w(BUILD_DEPS BUILD_TDEPS CFLAGS CPPFLAGS CXXFLAGS DEPS
                       FILES IDENT LDFLAGS LD_RUN_PATH MANIFEST TARGET TDEPS)

        EnvVars = %w(HAB_AUTH_TOKEN
                    HAB_CACHE_KEY_PATH
                    HAB_DEPOT_URL
                    HAB_ORG
                    HAB_ORIGIN
                    HAB_ORIGIN_KEYS
                    HAB_RING
                    HAB_RING_KEY
                    HAB_ROOT_PATH
                    HAB_STUDIOS_HOME
                    HAB_STUDIO_ROOT
                    HAB_USER)
    end


    module Utils
        # parse a ./results/last_build.env file, split lines on `=`
        # and return a hash containing all key/values
        def self.parse_last_build
            results = {}
            ## TODO: should we have a test root dir var?
            File.open("results/last_build.env", "r") do |f|
                f.each_line do |line|
                    chunks = line.split("=")
                    results[chunks[0].strip()] = chunks[1].strip()
                end
            end
            return results
        end

        class Ring
            attr_accessor :ctx
            attr_accessor :members

            def initialize(ctx, num_supervisors, package_to_run, group, org)
                @ctx = ctx
                listen_peer_port=9000
                sidecar_port=8000
                @members = []
                num_supervisors.times do |i|
                    if i > 0 then
                        my_peer_port = listen_peer_port - 1
                    else
                        my_peer_port = nil
                    end

                    member = HabTesting::Utils::RingMember.new(i,
                                                               @ctx,
                                                               package_to_run,
                                                               listen_peer_port,
                                                               sidecar_port,
                                                               group,
                                                               org,
                                                               my_peer_port)
                    puts member if @cmd_debug
                    @members << member
                    listen_peer_port += 1
                    sidecar_port += 1
                end
            end

            # put the ring into /mnt/doom
            def destroy()
                @members.each do |member|
                    member.stop()
                end
            end

            def start
                # TODO: wait until started!
                @members.each do |member|
                    member.start
                end
            end

            def stop(signal=9)
                @members.each do |member|
                    member.stop(signal)
                end
            end

            def wait_for_all_nodes_to_start
                @ctx.wait_for("nodes to start") do
                    begin
                        @members.each do |member|
                            member.get_status
                        end
                        true
                    rescue
                        false
                    end
                end
            end

            def wait_for_all_nodes_to_join
                @ctx.wait_for("all nodes to join") do
                    begin
                        gossip = @members[0].get_gossip
                        gossip["member_list"]["members"].length == 3
                    rescue
                        false
                    end
                end
            end

            def wait_until_node_stops(node_id)
                @ctx.wait_until_exception("member #{node_id} to stop") do
                    @members[node_id].get_status
                end
            end

            def wait_until_node_starts(node_id)
                @ctx.wait_until_no_exception("member #{node_id} to start") do
                    @members[node_id].get_status
                end
            end

        end

        class RingMember
            attr_accessor :ctx
            attr_accessor :group
            attr_accessor :http_port
            attr_accessor :id
            attr_accessor :org
            attr_accessor :package
            # someone elses supervisor port, we'll join to it
            attr_accessor :peer_port
            attr_accessor :pid
            attr_accessor :port

            def initialize(id, ctx, package, port, http_port, group, org, peer_port=nil)
                @ctx = ctx
                @group = group
                @http_port = http_port
                @id = id
                @org = org
                @package = package
                @peer_port = peer_port
                @port = port
            end

            def start
                cmdline = "#{@ctx.hab_bin} start #{@package}" \
                    " --listen-peer #{@ctx.public_ip}:#{@port}" \
                    " --listen-http #{@ctx.public_ip}:#{@http_port}" \
                    " --group #{@group} --org #{@org}"

                if not @peer_port.nil? then
                    # if we aren't the first, the join up to the previous sup
                    # that's been started
                    cmdline += " --peer #{@ctx.public_ip}:#{@peer_port}"
                end

                # TODO: redirecting output returns the shell pid, not the hab-sup pid
                # so it's hard to kill these (maybe kill the group?)
                #cmdline += " >> #{@ctx.log_file_name} 2>&1"

                puts "» Starting ring member #{@id}"
                puts "COMMAND LINE = #{cmdline}" if @ctx.cmd_debug
                # create a new process group to make the child easier to terminate
                @pid = spawn(cmdline, :pgroup => true, [:out, :err]=>@ctx.log_file_name)
                # have the ctx keep track of all children we spawn
                @ctx.all_children << [cmdline, @pid]
                puts "★ Started ring member #{@id}, pid #{@pid}, gossip port #{@port}, http port #{@http_port}"
            end

            def restart(signal=9)
                stop(signal)
                start()
            end

            # todo: pull out Process.kill and make a ctx.kill method
            # thats platform independent
            def stop(signal=9)
                puts "» Killing process #{@pid}"
                begin
                    Process.kill(signal, @pid)
                    Process.wait(@pid)
                rescue
                    puts "Failed to kill process #{@pid}"
                end
                puts "★ Killed process #{@pid}"

            end


            def http_get(path, as_json=true)
                http = Net::HTTP.new("#{@ctx.public_ip}", @http_port)
                request = Net::HTTP::Get.new(path)
                response = http.request(request)
                puts "RESPONSE BODY = #{response.body}" if @ctx.cmd_debug
                puts "RESPONSE CODE = #{response.code}" if @ctx.cmd_debug
                # TODO: look at response code etc
                if as_json then
                    JSON.parse(response.body)
                else
                    response
                end
            end

            def get_census
                http_get("/census")
            end

            def get_config
                # TODO: parse the toml?
                http_get("/config", false)
            end

            def get_election
                http_get("/election")
            end

            def get_gossip
                http_get("/gossip")
            end

            def get_health
                http_get("/health")
            end

            def get_status
                # this doesn't return valid json
                http_get("/status", false)
            end
        end

    end

    TestDir = Struct.new(:path, :caller)

    # The intent of the Platform class is to store any platform-independent
    # variables.
    class Platform
        include Singleton
        # path to the `hab` command
        attr_accessor :hab_bin

        # location to the Habitat key cache
        attr_accessor :hab_key_cache

        # A unique testing organization
        attr_accessor :hab_org

        # A unique testing origin
        attr_accessor :hab_origin

        # The path to installed packages, (ex: /hab/pkgs on Linux)
        attr_accessor :hab_pkg_path

        # A unique testing ring name
        attr_accessor :hab_ring

        # The path where running services are installed
        attr_accessor :hab_svc_path

        # path to the `hab-sup` command
        attr_accessor :hab_sup_bin

        # A unique testing user
        attr_accessor :hab_user

        # command output logs are stored in this directory
        attr_accessor :log_dir

        # The filename currently be used to log command output.
        # This file is stored in @log_dir
        attr_accessor :log_name

        # if true, display command output
        attr_accessor :cmd_debug

        # default timeout for child processes before failing
        attr_accessor :cmd_timeout_seconds

        # if there is an example failure, don't cleanup the state on
        # disk if @cleanup is set to false
        attr_accessor :cleanup

        # a unique filename that contains a "cleanup script",
        # that is, a script that will clean up after your test
        # upon failed
        attr_accessor :cleanup_filename

        # for any command that spawn child processes, we can use
        # all_children to store pid info to see if we're leaving any
        # processes running after tests have completed
        attr_accessor :all_children

        # we allow each test to register a set of directories and/or files
        # that are generated as part of testing.
        # We'll clean up directories upon completion if tests pass.
        attr_accessor :test_directories

        # a known origin name for testing, we won't ever publish keys
        # for this origin, as keys will be generated at the start of
        # a test run if they do not exist.
        attr_accessor :shared_test_origin


        attr_accessor :public_ip

        def initialize
            @all_children = []
            @cleanup = true
            @cmd_debug = false
            @cmd_timeout_seconds = 30
            @test_directories = []
            @shared_test_origin = "hab_test"
        end

        # generate a unique name for use in testing
        def unique_name()
            SecureRandom.uuid
        end


        # display testing parameters upon startup
        def banner()
            puts "→ Test params:"
            self.instance_variables.sort.each do |k|
                puts "\t #{k[1..-1]} = #{self.instance_variable_get(k)}"
            end
            puts "→ Logging command output to #{self.log_file_name()}"
        end



        # Common setup for tests, including setting a test origin
        # and key generation.
        def common_setup
            get_public_ip

            if not ENV['HAB_TEST_DEBUG'].nil? then
                puts "★ Debugging enabled"
                @cmd_debug = true
            end
            ENV['HAB_ORIGIN'] = @hab_origin

            create_hab_test_origin

            puts "» Generating origin key"
            cmd_expect("origin key generate #{@hab_origin}",
                       "Generated origin key pair #{@hab_origin}")
            puts "★ Generated origin key"

            puts "» Generating user key"
            cmd_expect("user key generate #{@hab_user}",
                       "Generated user key pair #{@hab_user}")
            puts "★ Generated user key"

            puts "» Generating ring key"
            cmd_expect("ring key generate #{@hab_ring}",
                       "Generated ring key pair #{@hab_ring}")
            puts "★ Generated ring key"
            # we don't generate a service key here because they depend
            # on the name of the service that's being run

            # your first run will probably be slower as we need
            # to build the simple_service fixture.
            # Subsequent restarts of the tests will pickup
            # the installed package.
            build_and_install_shared_fixture
            puts "★ Setup complete"
            puts "-" * 80
        end

        # Common teardown for tests
        def common_teardown
            # display all pids that were created with cmd + cmd_expect
            if @cmd_debug
                @all_children.each do |pidinfo|
                    puts "PID INFO: #{pidinfo}"
                end
            end
        end

        def build_and_install_shared_fixture

            installed_path = Pathname.new(@hab_pkg_path).
                               join(@shared_test_origin).
                               join("simple_service")
            if installed_path.exist? then
                puts "★ simple_service is already installed"
                return
            end
            # /hab/pkgs/hab_test/simple_service/
            prev_origin = ENV['HAB_ORIGIN']

            begin
                ENV['HAB_ORIGIN'] = @shared_test_origin
                puts "» Building simple_service test fixture"
                # building a package can take quite awhile, let's bump the timeout to
                # 60 seconds to be sure we finish in time.
                cmd_expect("studio build fixtures/simple_service",
                                            "I love it when a plan.sh comes together",
                                            :timeout_seconds => 60)
                puts "★ Built simple_service test fixture"

                puts "» Installing simple_service test fixture"
                cmd_expect("pkg install ./results/hab_test-simple_service*.hart",
                           "complete")
                puts "★ simple_service installed"
            ensure
                ENV['HAB_ORIGIN'] = prev_origin
            end

        end

        def register_dir(d)
            c = caller[0].match(/(.+):(\d+)/)[1..2].join(" line ")
            @test_directories << TestDir::new(d, c)
        end

        ## This method waits for a block to return true,
        ## otherwise it times out with either a max # of retries,
        ## or a given timeout
        def wait_for(title, **wait_options, &block)
            debug = wait_options[:debug] || false
            max_retries = wait_options[:max_retries] || 100
            max_wait_seconds = wait_options[:max_wait_seconds] || 30
            sleep_increment_seconds = wait_options[:sleep_increment_seconds] || 1
            show_progress = wait_options[:show_progress] || true

            puts "debug = #{debug}" if debug
            puts "sleep_increment_seconds = #{sleep_increment_seconds}" if debug

            puts "Waiting for #{title}, max retries = #{max_retries}, max_wait_seconds = #{max_wait_seconds}"
            retries = 0
            # throw an exception if the block if we've waited longer
            # that max_wait_seconds
            Timeout::timeout(max_wait_seconds) do
                while retries < max_retries
                    puts "Calling block [retry #{retries} of #{max_retries}]" if debug
                    result = block.call
                    puts "block result = #{result}" if debug
                    print "." if show_progress
                    if result then
                        puts " -> Success" if show_progress
                        return result
                    else
                        retries += 1
                    end
                    sleep(sleep_increment_seconds)
                end
                puts "" if show_progress
                if retries >= max_reties
                    throw "#{title} failed after #{retries} retries"
                end
            end
        end

        def wait_until_exception(title, **wait_options, &block)
            wait_for(title, wait_options) do
                begin
                    yield block
                    false
                rescue
                    true
                end
            end
        end

        def wait_until_no_exception(title, **wait_options, &block)
            wait_for(title, wait_options) do
                begin
                    yield block
                    true
                rescue
                    false
                end
            end
        end


        # convenience method for creating a ring and "link" it to
        # a ctx
        def create_ring(num_supervisors, package_to_run, group, org)
            return HabTesting::Utils::Ring.new(self, num_supervisors, package_to_run, group, org)
        end
    end

    class LinuxPlatform < Platform
        attr_accessor :ip_path
        attr_accessor :awk_path

        def initialize
            super
            @cleanup_filename = "cleanup_#{SecureRandom.hex(10)}.sh"
            @hab_bin="/src/target/debug/hab"
            @hab_key_cache = "/hab/cache/keys"
            @hab_org = "org_#{unique_name()}"
            @hab_origin = "origin_#{unique_name()}"
            @hab_pkg_path = "/hab/pkgs"
            @hab_ring = "ring_#{unique_name()}"
            @hab_sup_bin = "/src/target/debug/hab-sup"
            @hab_svc_path = "/hab/svc"
            @hab_user = "user_#{unique_name()}"
            @log_dir = "./logs"
            @log_name = "hab_test-#{Time.now.utc.iso8601.gsub(/\:/, '-')}.log"


            @ip_path = "/sbin/ip"
            @awk_path = "/usr/bin/awk"
            banner()
        end

        def get_public_ip
            puts "» Detecting IP"
            ip = `#{@ip_path} route get 8.8.8.8 | #{@awk_path} '{printf "%s", $NF; exit}'`
            # parse the result, it should be an ip
            # if not, use this low-tech solution to make the tests fail
            IPAddr.new(ip)
            puts "★ Detected IP address as #{ip}"
            @public_ip = ip
        end

        def common_setup
            super
        end


        # this function will either run all commands contained within
        # if the tests pass, OR generate a cleanup.sh script
        # so you can clean the env yourself after investigating
        # why tests failed.
        def common_teardown
            super
            # util function that will run our cleanup commands for
            # us unless tests have failed. If tests have failed,
            # then we generate a cleanup.sh script so you don't
            # have to clean things up manually.
            def exec(command)
                if @cleanup then
                    c = Mixlib::ShellOut.new(command)
                    c.run_command
                    c.error!
                else
                    # this is lame, sorry
                    open(@cleanup_filename, 'a') { |f|
                        f.puts command
                    }
                end
            end

            if not @cleanup
                Mixlib::ShellOut.new("rm -f #{@cleanup_filename}").run_command().error!
                Mixlib::ShellOut.new("touch #{@cleanup_filename}").run_command().error!
                Mixlib::ShellOut.new("chmod 700 #{@cleanup_filename}").run_command().error!
            end

            exec "unset HAB_ORIGIN"
            # remove generated keys
            exec "rm -f #{Pathname.new(@hab_key_cache).join(@hab_origin)}*"
            exec "rm -f #{Pathname.new(@hab_key_cache).join(@hab_user)}*"
            exec "rm -f #{Pathname.new(@hab_key_cache).join(@hab_ring)}*"
            #TODO: once we have service keys, we'll have to remove them here
            @test_directories.each do |d|
                exec "echo \"Removing : #{d.path}\""
                exec "echo \"   registered in: #{d.caller}\""
                exec "rm -rf #{d.path}"
            end

            if not @cleanup
                puts "WARNING: not cleaning up testing environment"
                puts "Please run #{@cleanup_filename} manually"
            end

        end


        def show_env()
            puts "X" * 80
            puts `env`
            puts "X" * 80
        end

        # execute a `hab` subcommand and wait for the process to finish
        def cmd(cmdline, **cmd_options)
            debug = cmd_options[:debug] || @cmd_debug

            if debug then
                puts "X" * 80
                puts `env`
                puts "X" * 80
            end

            fullcmdline = "#{@hab_bin} #{cmdline} | tee -a #{log_file_name()} 2>&1"
            # record the command we'll be running in the log file
            `echo #{fullcmdline} >> #{log_file_name()}`
            puts " → #{fullcmdline}"
            # TODO: replace this with Mixlib:::Shellout
            pid = spawn(fullcmdline)
            @all_children << [cmdline, pid]
            Process.wait pid
            return $?
        end

        # runs a command in the background, returns a pid
        # DON'T THIS, it has subtle bugs because of the | tee
        # and pids
        #def bg_cmd(cmdline, **cmd_options)
        #    debug = cmd_options[:debug] || @cmd_debug

        #    if debug then
        #        puts "X" * 80
        #        puts `env`
        #        puts "X" * 80
        #    end

        #    fullcmdline = "#{@hab_bin} #{cmdline} | tee -a #{log_file_name()} 2>&1"
        #    # record the command we'll be running in the log file
        #    `echo #{fullcmdline} >> #{log_file_name()}`
        #    puts " → #{fullcmdline}"
        #    # TODO: replace this with Mixlib:::Shellout
        #    pid = spawn(fullcmdline)
        #    @all_children << [cmdline, pid]
        #    return pid
        #end


        # create a common key that we use for testing only.
        # These keys should never be uploaded to the Depot, and
        # they're only generated if needed.
        # It's ok if these keys stick around in between tests.
        def create_hab_test_origin
            re = Regexp.new(@shared_test_origin)

            skip = Dir.entries(@hab_key_cache).any? do |f|
                re =~ f
            end

            if skip then
                puts "★ #{shared_test_origin} keys already exists"
            else
                puts "» Generating #{@shared_test_origin} origin key"
                cmd_expect("origin key generate #{@shared_test_origin}",
                        "Generated origin key pair")
                puts "★ Generated #{@shared_test_origin} origin key"
            end
        end



        # execute a possibly long-running process and wait for a particular string
        # in it's output. If the output is found, kill the process and return
        # it's exit status. Otherwise, raise an exception so specs fail quickly.
        def cmd_expect(cmdline, desired_output, **cmd_options)
            puts "X" * 80 if @cmd_debug
            puts cmd_options if @cmd_debug
            debug = cmd_options[:debug] || @cmd_debug

            timeout = cmd_options[:timeout_seconds] || @cmd_timeout_seconds
            kill_when_found = cmd_options[:kill_when_found] || false
            show_env() if debug
            # passing output to | tee
            #fullcmdline = "#{@hab_bin} #{cmdline} | tee -a #{log_file_name()} 2>&1"
            fullcmdline = "#{@hab_bin} #{cmdline}"
            # record the command we'll be running in the log file
            `echo #{fullcmdline} >> #{log_file_name()}`
            puts " → #{fullcmdline}"

            output_log = open(log_file_name(), 'a')
            begin
                Open3.popen3(fullcmdline) do |stdin, stdout, stderr, wait_thread|
                    @all_children << [fullcmdline, wait_thread.pid]
                    puts "Started child process id #{wait_thread[:pid]}" if debug
                    found = false
                    begin
                        Timeout::timeout(timeout) do
                            loop do
                                line = stdout.readline()
                                output_log.puts line
                                puts line if debug
                                if line.include?(desired_output) then
                                    if kill_when_found then
                                        puts "Sending a KILL to child process #{wait_thread.pid}" if debug
                                        Process.kill('KILL', wait_thread.pid)
                                        found = true
                                        break
                                    else
                                        puts "Output found but not sending signal to child" if debug
                                        # let the process finish, or timeout
                                        found = true
                                    end
                                end
                            end
                        end
                    rescue EOFError
                        if found then
                            puts "Found value as process finished" if debug
                            return wait_thread.value
                        else
                            raise "Process finished without finding desired output: #{desired_output}"
                        end
                    rescue Timeout::Error
                        # TODO: do timeouts always return failure?
                        puts "Timeout" if debug
                        Process.kill('KILL', wait_thread.pid)
                        puts "Child process killed" if debug
                        raise "Process timeout waiting for desired output: #{desired_output}"
                    ensure
                        output_log.close()
                    end

                    if found == true then
                        puts "\tFound: #{desired_output}" if debug
                        return wait_thread.value
                    else
                        raise "Output not found: #{desired_output}"
                    end
                end
            end
        end

        # generate a unique log file name in the given log_dir
        def log_file_name()
            File.join(@log_dir, @log_name)
        end

    end

    class WindowsPlatform
        def initialize
            # :-(
            raise "Windows platform not implemented"
        end
    end

end # module HabTesting
