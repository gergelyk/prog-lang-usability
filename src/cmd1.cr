cmd = Process.new("ls", args: ["/"],
  output: Process::Redirect::Pipe,
  error: Process::Redirect::Pipe)

stdout = cmd.output.gets_to_end
stderr = cmd.error.gets_to_end
status = cmd.wait
if status.exit_status == 0
  puts stdout.split
else
  raise stderr
end
