#!/usr/bin/env ruby

require 'tmpdir'
require 'json'

usage = "Usage: #{__FILE__} flattened.rs"

if !ARGV[0] then
	p usage
	exit
end

f = ARGV[0]

Dir.mktmpdir do |dir|
	FileUtils.cp(f, File.join(dir, "main.rs"))
	Dir.chdir(dir)

	while true do
		cmd = "rustc main.rs --error-format=json"

		msg = ""
		IO.popen(cmd, :err => [:child, :out]) {|io|
			msg = io.gets
			io.close_write
		}
		break unless msg  # No error or warning

		remove = []
		msg.split("\n").each{|line|
			json = JSON.parse(line)
			# STDERR.puts JSON.pretty_generate(json)
			next unless /(unresolved import|unused import|function is never used)/.match(json["message"])

			json["spans"].each {|e|
				from = e["line_start"].to_i
				to = e["line_end"].to_i  # inclusive
				remove.push([from, to])
			}
		}
		break if remove.empty?
		i = 1
		tmp = File.open("tmp.rs", "w")
		File.open("main.rs") do |f| f.each_line {|c|
			ok = true
			remove.each { |from, to|
				ok = false if from <= i && i <= to 
			}
			tmp.write c if ok

			i += 1
		}
		end
		tmp.close
		FileUtils.mv("tmp.rs", "main.rs")
	end
	puts File.open("main.rs").readlines()
end
