import typo;
import subprocess;
from termcolor import cprint;

success = 0
failure = 0
error = 0

charSwapSuccess = 0
missingCharSuccess = 0
extraCharSuccess = 0
nearbyCharSuccess = 0
skippedSpaceSuccess = 0
repeatedCharSuccess = 0
unicharSuccess = 0

def run(input, expected):
    global success
    global failure
    global error
    result = subprocess.run([executable, '-c', f'{input}', '-p'], capture_output=True)
    if not result.stderr:
        correction = result.stdout.decode().strip()
        if correction == expected or input == expected:
            success += 1
            return ('success', None)
        else:
            failure += 1;
            return ('failure', correction)
    else:
        error += 1
        return ('error', result.stderr)

def getColor(result):
    match result:
        case 'success':
            return 'green'
        case 'failure':
            return 'yellow'
        case 'error':
            return 'red'

def test(line):
    global charSwapSuccess
    global missingCharSuccess
    global extraCharSuccess
    global nearbyCharSuccess
    global skippedSpaceSuccess
    global repeatedCharSuccess
    global unicharSuccess

    print(f'Testing: "{line}" ...\n')
    charSwapErrorInput = typo.StrErrer(line).char_swap().result
    result = run(charSwapErrorInput, line)
    color = getColor(result[0])
    cprint(f'Character swap: {charSwapErrorInput} ... {result[0]}', color)
    if result[0] == 'success':
        charSwapSuccess += 1
    else:
        cprint(f'\tgot: {result[1]}', color)

    missingCharErrorInput = typo.StrErrer(line).missing_char().result
    result = run(missingCharErrorInput, line)
    color = getColor(result[0]) 
    cprint(f'Missing character: {missingCharErrorInput} ... {result[0]}', color)
    if result[0] == 'success':
        missingCharSuccess += 1
    else:
        cprint(f'\tgot: {result[1]}', color)

    extraCharErrorInput = typo.StrErrer(line).extra_char().result
    result = run(extraCharErrorInput, line)
    color = getColor(result[0]) 
    cprint(f'Extra character: {extraCharErrorInput} ... {result[0]}', color)
    if result[0] == 'success':
        extraCharSuccess += 1
    else:
        cprint(f'\tgot: {result[1]}', color)

    nearbyCharErrorInput = typo.StrErrer(line).nearby_char().result
    result = run(nearbyCharErrorInput, line)
    color = getColor(result[0])
    cprint(f'Nearby character: {nearbyCharErrorInput} ... {result[0]}', color)
    if result[0] =='success':
        nearbyCharSuccess += 1
    else:
        cprint(f'\tgot: {result[1]}', color)

    skippedSpaceErrorInput = typo.StrErrer(line).skipped_space().result
    result = run(skippedSpaceErrorInput, line)
    color = getColor(result[0])
    cprint(f'Skipped space: {skippedSpaceErrorInput} ... {result[0]}', color)
    if result[0] == 'success':
        skippedSpaceSuccess += 1
    else:
        cprint(f'\tgot: {result[1]}', color)

    repeatedCharErrorInput = typo.StrErrer(line).repeated_char().result
    result = run(repeatedCharErrorInput, line)
    color = getColor(result[0]) 
    cprint(f'Repeated character: {repeatedCharErrorInput} ... {result[0]}', color)
    if result[0] == 'success':
        repeatedCharSuccess += 1
    else:
        cprint(f'\tgot: {result[1]}', color)

    unicharErrorInput = typo.StrErrer(line).unichar().result
    result = run(unicharErrorInput, line)
    color = getColor(result[0])
    cprint(f'Unicharacter: {unicharErrorInput} ... {result[0]}', color)
    if result[0] == 'success':
        unicharSuccess += 1
    else:
        cprint(f'\tgot: {result[1]}', color)

    print()

testCommandLines = [
    'awk \'{print $5}\' file',
    'awk \'/foo/ {print $2}\' file > output',
    'cat --number file',
    'cat file1 file2 > output',
    'cd dir',
    'cd ..',
    'chmod u+x file',
    'chmod --recursive g+w,o+w dir',
    'chown user file',
    'chown --no-dereference user symlink',
    'clear',
    'clear -x',
    'cp file dest',
    'cp --verbose --recursive dir dest',
    'curl https://example.com',
    'curl --location --dump-header - https://example.com',
    'date +%c',
    'date --date @1473305798',
    'dd if=file.iso of=/dev/drive status=progress',
    'dd bs=4M conv=fsync if=/dev/drive of=/dev/dest'
    'df',
    'df --human-readable',
    'diff file1 file2',
    'diff --ignore-all-space file1 file2',
    'echo "Hello World" >> file',
    'echo "path: $PATH"',
    'exit',
    'exit 1',
    'file path',
    'file --uncompress path',
    'find file -name "*.ext"',
    'find file -path "*/path/*/*.ext" -or -name "*pattern*"',
    'ftp ftp.example.com',
    'ftp 192.168.1 34',
    'git init',
    'git add --all',
    'grep "pattern" file',
    'grep --fixed-strings "string" file',
    'gzip file',
    'gzip --decompress file',
    'head file',
    'head --lines 5 file',
    'history',
    'history 20 | wc',
    'ifconfig name',
    'ifconfig -a',
    'join file1 file2',
    'join -t ',' file1 file2',
    'kill pid',
    'kill --table',
    'less file',
    'less file | cat',
    'ln --symbolic path symlink',
    'ln --symbolic --force path symlink',
    'ls --all',
    'ls -l --human-readable',
    'man command',
    'man --where command',
    'mkdir dir',
    'mkdir --parents dir',
    'more file',
    'more +10 file',
    'mv source target',
    'mv --no-clobber source target',
    'nano file',
    'nano --ignorercfiles',
    'ping host',
    'ping -c 10 host',
    'ps aux | grep string',
    'ps --sort size',
    'pwd',
    'pwd --physical',
    'rm file',
    'rm --recursive dir',
    'ls | sed "s/apple/mango/g"',
    'sed --in-place "s/apple/mango/g"',
    'shutdown -h now',
    'shutdown --reboot now',
    'sleep 10',
    'sleep 30 && ls --all',
    'sort file',
    'sort --reverse file',
    'ssh username@host',
    'ssh -i file username@host',
    'tail file',
    'tail --lines 5 file',
    'tar cf target file',
    'tar xvf source',
    'top',
    'top --idle-toggle',
    'touch file',
    'touch --no-create -m file',
    'unzip file',
    'unzip -l file',
    'uptime',
    'uptime --pretty',
    'wc --lines file',
    'wc --words file',
    'wget https://example.com',
    'wget --user name --password password https://example.com',
]

executable = '../target/release/cli_assist'

for commandLine in testCommandLines:
    test(commandLine)
print('===========================================================\n')

print(f'Character swap: {charSwapSuccess}/{len(testCommandLines)}')
print(f'Missing character: {missingCharSuccess}/{len(testCommandLines)}')
print(f'Extra character: {extraCharSuccess}/{len(testCommandLines)}')
print(f'Nearby character: {nearbyCharSuccess}/{len(testCommandLines)}')
print(f'Skipped space: {skippedSpaceSuccess}/{len(testCommandLines)}')
print(f'Repeated character: {repeatedCharSuccess}/{len(testCommandLines)}')
print(f'Unicharacter: {unicharSuccess}/{len(testCommandLines)}')

print('\n===========================================================\n')

print('TOTAL')
print(f'Successes: {success}')
print(f'Failures: {failure}')
print(f'Errors: {error}')

