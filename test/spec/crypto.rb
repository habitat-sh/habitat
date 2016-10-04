# TODO: .rspec file doesn't seem to be honored, so we need
# to manually include the spec_helper here
require_relative 'spec_helper'

require 'tempfile'

describe "Habitat crypto" do
    before(:all) do
        ctx.common_setup()
    end

    after(:all) do
        ctx.common_teardown()
    end

    after(:each) do |example|
        if example.exception
            puts "Detected failed examples, keeping environment"
            ctx.cleanup = false
        end
    end

    context "artifact signing and verification" do
        it "should work round-trip" do
            file = Tempfile.new('file_to_sign')

            File.open(file, 'w') do |file|
                file.write("This text will be signed")
            end

            signed_file="#{file.path}.signed"

            result = ctx.hab_cmd_expect("pkg sign #{file.path} #{signed_file} --origin #{ctx.hab_origin}",
                                    "Signed artifact")

            expect(result.exited?).to be true
            expect(result.exitstatus).to eq 0

            result = ctx.hab_cmd_expect("pkg verify #{signed_file}",
                                    "Verified artifact")
            expect(result.exited?).to be true
            expect(result.exitstatus).to eq 0
        end
    end
end
