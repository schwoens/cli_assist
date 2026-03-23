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
randomSpaceSuccess = 0
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
    global randomSpaceSuccess
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

    randomSpaceErrorInput = typo.StrErrer(line).random_space().result
    result = run(randomSpaceErrorInput, line)
    color = getColor(result[0]) 
    cprint(f'Random space: {randomSpaceErrorInput} ... {result[0]}', color)
    if result[0] == 'success':
        randomSpaceSuccess += 1
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
    'ls --all',
    'ls --almost-all | cat | wc',
    'ls --almost-all > output',
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
print(f'Random space: {randomSpaceSuccess}/{len(testCommandLines)}')
print(f'Repeated character: {repeatedCharSuccess}/{len(testCommandLines)}')
print(f'Unicharacter: {unicharSuccess}/{len(testCommandLines)}')

print('\n===========================================================\n')

print('TOTAL')
print(f'Successes: {success}')
print(f'Failures: {failure}')
print(f'Errors: {error}')

