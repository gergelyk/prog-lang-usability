stdout = `ls /`
if $?.success? # return code != 0 ?
  puts stdout.split
end
