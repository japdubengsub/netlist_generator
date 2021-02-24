import os
import subprocess

my_env = os.environ.copy()


# print cpu
# print kernel
# print rust ver


###############
# BENCH
###############
# my_env["RUSTFLAGS"] = "-C target-cpu=native"
cmd = "cargo +nightly bench".split()
# subprocess.run(cmd, env=my_env)
# exit()

###############
# BUILD
###############

my_env["RUSTFLAGS"] = "-C force-frame-pointers=y " + "-C target-cpu=native "
#cmd = "RUSTFLAGS='-C force-frame-pointers=y' cargo build --release"
cmd = "cargo build --release".split()
# subprocess.run(cmd, env=my_env)

############################

program = './target/release/netlist-generator'
# program = '/home/rbr/Public/target/release/netlist-generator'

files = [
    #r'./dump1.csv',
    r'dump2.csv',
    # r'dump3.csv',
]

params = [
    #'',
    #'-r 10000',
    #'-r 10_000 -m 24',
    #'-r 100_000',
    #'-r 100_000 -m 24',
    '-r 300000',
    #'-r 300000 -m 24',
]

tools = [
    # '',
    # '/usr/bin/time',
    # 'time',
    # 'valgrind --tool=massif --pages-as-heap=yes',
    # 'valgrind --tool=massif --stacks=yes',
    # 'perf record -g',
    'perf stat -d -B',
    # 'valgrind --leak-check=full --show-leak-kinds=all',
    # 'perf report -I --header',
    # 'perf report -g graph,0.5,caller',
]

for f in files:
    for p in params:
        for t in tools:
            print("="*120)

            cmd = [*t.split(" "), program, '-i', f, '-o', f[:-3] + 'txt', *p.split(" ")]
            # cmd = ' '.join(cmd)
            cmd = '/bin/bash -c "' + ' '.join(cmd) + '"'
            print(cmd)

            log_file = '__'.join([f, *p.split(" "), *t.split(" ")]) + '.log'
            log_file = log_file.replace('/', '_')
            print(log_file)

            result = subprocess.run(
                cmd,
                shell=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                # capture_output=True
                # ).stdout.decode('utf-8')
            ).stdout

            try:
                result = result.decode('utf-8')
            except:
                result = str(result)

            with open(log_file, mode='wt+') as logfile:
                logfile.write(result)
