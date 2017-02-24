#!/usr/bin/env ruby

def mod_path(a)
	err_msg = "File for #{a} not found"
	if a.size == 0 then
		if File.exist?("lib.rs") then
			return "lib.rs"
		end
		p err_msg
		exit
	end
	path = File.join a
	if File.exist?(path + ".rs") then
		return path + ".rs"
	elsif File.exist?(File.join(path, "mod.rs")) then
		return File.join(path, "mod.rs")
	end
	p err_msg
	exit
end

# ["path", "to", "target"]
# path/to/target.rs or path/to/target/mod.rs
def do_it(a)
	path = mod_path(a)
	File.open(path).each_line do |line|
		line.chomp!
		if m = /mod (?<name>\w+);/.match(line) then
			name = m[:name]
			puts "pub mod #{name} {"
			do_it(a.clone().push(name))
			puts "}"
		else
			puts line
		end
	end
end

usage = "Usage: #{__FILE__} package_name"

if !ARGV[0] then
	puts usage
	exit
end

package = ARGV[0]
puts "mod #{package} {"
do_it([])
puts "}"

File.open("main.rs").each_line {|line|
	puts line unless /extern crate/.match(line)
}

