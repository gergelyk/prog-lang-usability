import subprocess

target_dir = "/"
status = subprocess.run(["ls", target_dir],
                        stdout=subprocess.PIPE,
                        stderr=subprocess.PIPE)

if status.returncode:
    msg = status.stderr.decode()
    raise Exception(msg)
else:
    list_files = status.stdout.decode().split()
    print(list_files)
