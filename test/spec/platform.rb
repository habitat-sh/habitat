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
require 'mixlib/shellout'
require 'open3'
require 'pathname'
require 'securerandom'
require 'shellwords'
require 'singleton'
require 'time'
require 'timeout'

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

        def initialize
            @all_children = []
            @cleanup = true
            @cmd_debug = false
            @cmd_timeout_seconds = 30
            @test_directories = []
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
            if not ENV['HAB_TEST_DEBUG'].nil? then
                puts "★ Debugging enabled"
                @cmd_debug = true
            end
            ENV['HAB_ORIGIN'] = @hab_origin
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

            puts "debug = #{debug}"
            puts "sleep_increment_seconds = #{sleep_increment_seconds}"

            puts "Waiting for #{title}, max retries = #{max_retries}, max_wait_seconds = #{max_wait_seconds}"
            retries = 0
            # throw an exception if the block if we've waited longer
            # that max_wait_seconds
            Timeout::timeout(max_wait_seconds) do
                while retries < max_retries
                    puts "Calling block [retry #{retries} of #{max_retries}]" if debug
                    result = block.call
                    puts "block result = #{result}" if debug
                    print "*" if show_progress
                    if result then
                        return result
                    else
                        retries += 1
                    end
                    sleep(sleep_increment_seconds)
                end
                print "" if show_progress
                if retries >= max_reties
                    throw "#{title} failed after #{retries} retries"
                end
            end

        end
    end

    class LinuxPlatform < Platform

        def initialize
            super
            @cleanup_filename = "cleanup_#{SecureRandom.hex(10)}.sh"
            @hab_bin="/src/components/hab/target/debug/hab"
            @hab_key_cache = "/hab/cache/keys"
            @hab_org = "org_#{unique_name()}"
            @hab_origin = "origin_#{unique_name()}"
            @hab_pkg_path = "/hab/pkgs"
            @hab_ring = "ring_#{unique_name()}"
            @hab_sup_bin = "/src/components/sup/target/debug/hab-sup"
            @hab_svc_path = "/hab/svc"
            @hab_user = "user_#{unique_name()}"
            @log_dir = "./logs"
            @log_name = "hab_test-#{Time.now.utc.iso8601.gsub(/\:/, '-')}.log"

            banner()
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

        def show_env()
            puts "X" * 80
            puts `env`
            puts "X" * 80
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
