import os
import subprocess

my_env = os.environ.copy()


print cpu
print kernel
print rust ver


###############
# BENCH
###############
# my_env["RUSTFLAGS"] = "-C target-cpu=native"
# cmd = "cargo +nightly bench".split()
# subprocess.run(cmd, env=my_env)


###############
# BUILD
###############

my_env["RUSTFLAGS"] = "-C force-frame-pointers=y -C target-cpu=native"

#cmd = "RUSTFLAGS='-C force-frame-pointers=y' cargo build --release"
cmd = "cargo build --release".split()
# subprocess.run(cmd, env=my_env)


program = './target/release/netlist-generator'

files = [
    #r'./dump1.csv',
    #r'dump2.csv',
    r'dump3.csv',
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
    '',
    'time',
    'valgrind --tool=massif --pages-as-heap=yes',
    'valgrind --tool=massif --stacks=yes',
    'perf record -g',
    'perf stat -B',
    'valgrind --leak-check=full --show-leak-kinds=all',
    # 'perf report',
    #'perf report -g graph,0.5,caller',
]

for f in files:
    for p in params:
        for t in tools:
            cmd = [*t.split(" "), program, '-i', f, '-o', f[:-3] + 'txt', *p.split(" ")]
            # print(cmd)
            log_file = '__'.join([f, *p.split(" "), *t.split(" ")]) + '.log'
            print(log_file)

            # result = subprocess.run(cmd, shell=True, stdout=subprocess.PIPE).stdout.decode('utf-8')

            # with open(log_file, mode='wt+') as logfile:
            #     logfile.write(result)
