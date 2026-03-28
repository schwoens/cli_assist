import typo;
import subprocess;
from termcolor import cprint;

success = 0
noCorrection = 0
partialCorrection = 0
error = 0
skipped = 0

charSwapSuccesses = 0
missingCharSuccesses = 0
extraCharSuccesses = 0
nearbyCharSuccesses = 0
skippedSpaceSuccesses = 0
randomSpaceSuccesses = 0
repeatedCharSuccesses = 0
unicharSuccesses = 0

charSwapPartialSuccesses = 0
missingCharPartialSuccesses = 0
extraCharPartialSuccesses = 0
nearbyCharPartialSuccesses = 0
skippedSpacePartialSuccesses = 0
randomSpacePartialSuccesses = 0
repeatedCharPartialSuccesses = 0
unicharPartialSuccesses = 0

charSwapSkips = 0
missingCharSkips = 0
extraCharSkips = 0
nearbyCharSkips = 0
skippedSpaceSkips = 0
randomSpaceSkips = 0
repeatedCharSkips = 0
unicharSkips = 0

def run(input, expected):
    global success
    global noCorrection
    global partialCorrection
    global error
    global skipped
    result = subprocess.run([executable, '-c', f"'{input}'", '-p'], capture_output=True)
    if not result.stderr:
        correction = result.stdout.decode().strip()
        if input == expected:
            skipped += 1
            return ('skipped', None)
        elif correction == expected:
            success += 1
            return ('success', None)
        elif correction == 'No correction available':
            noCorrection += 1
            return ('no correction', correction)
        else:
            partialCorrection += 1;
            return ('partial success', correction)
    else:
        error += 1
        return ('error', result.stderr)

def get_color(result):
    match result:
        case 'success':
            return 'green'
        case 'no correction':
            return 'yellow'
        case 'partial success':
            return 'magenta'
        case 'error':
            return 'red'
        case 'skipped':
            return 'light_grey'

def test(line):
    global charSwapSuccesses
    global missingCharSuccesses
    global extraCharSuccesses
    global nearbyCharSuccesses
    global skippedSpaceSuccesses
    global randomSpaceSuccesses
    global repeatedCharSuccesses
    global unicharSuccesses

    global charSwapPartialSuccesses
    global missingCharPartialSuccesses
    global extraCharPartialSuccesses
    global nearbyCharPartialSuccesses
    global skippedSpacePartialSuccesses
    global randomSpacePartialSuccesses
    global repeatedCharPartialSuccesses
    global unicharPartialSuccesses

    global charSwapSkips
    global missingCharSkips
    global extraCharSkips
    global nearbyCharSkips
    global skippedSpaceSkips
    global randomSpaceSkips
    global repeatedCharSkips
    global unicharSkips

    print(f'{line}; length: {len(line)}\n')
    charSwapErrorInput = typo.StrErrer(line).char_swap().result
    result = run(charSwapErrorInput, line)
    color = get_color(result[0])
    cprint(f'character swap: {charSwapErrorInput} ... {result}', color)
    match result[0]:
        case 'success':
            charSwapSuccesses += 1
        case 'skipped':
            charSwapSkips += 1
        case 'partial success':
            charSwapPartialSuccesses += 1

    missingCharErrorInput = typo.StrErrer(line).missing_char().result
    result = run(missingCharErrorInput, line)
    color = get_color(result[0]) 
    cprint(f'missing character: {missingCharErrorInput} ... {result}', color)
    match result[0]:
        case 'success':
            missingCharSuccesses += 1
        case 'skipped':
            missingCharSkips += 1
        case 'partial success':
            missingCharPartialSuccesses += 1

    extraCharErrorInput = typo.StrErrer(line).extra_char().result
    result = run(extraCharErrorInput, line)
    color = get_color(result[0]) 
    cprint(f'extra character: {extraCharErrorInput} ... {result}', color)
    match result[0]:
        case 'success':
            extraCharSuccesses += 1
        case 'skipped':
            extraCharSkips += 1
        case 'partial success':
            extraCharPartialSuccesses += 1

    nearbyCharErrorInput = typo.StrErrer(line).nearby_char().result
    result = run(nearbyCharErrorInput, line)
    color = get_color(result[0])
    cprint(f'nearby character: {nearbyCharErrorInput} ... {result}', color)
    match result[0]:
        case 'success':
            nearbyCharSuccesses += 1
        case 'skipped':
            nearbyCharSkips += 1
        case 'partial success':
            nearbyCharPartialSuccesses += 1

    skippedSpaceErrorInput = typo.StrErrer(line).skipped_space().result
    result = run(skippedSpaceErrorInput, line)
    color = get_color(result[0])
    cprint(f'skipped space: {skippedSpaceErrorInput} ... {result}', color)
    match result[0]:
        case 'success':
            skippedSpaceSuccesses += 1
        case 'skipped':
            skippedSpaceSkips += 1
        case 'partial success':
            skippedSpacePartialSuccesses += 1

    randomSpaceErrorInput = typo.StrErrer(line).random_space().result
    result = run(randomSpaceErrorInput, line)
    color = get_color(result[0]) 
    cprint(f'random space: {randomSpaceErrorInput} ... {result}', color)
    match result[0]:
        case 'success':
            randomSpaceSuccesses += 1
        case 'skipped':
            randomSpaceSkips += 1
        case 'partial success':
            randomSpacePartialSuccesses += 1

    repeatedCharErrorInput = typo.StrErrer(line).repeated_char().result
    result = run(repeatedCharErrorInput, line)
    color = get_color(result[0]) 
    cprint(f'repeated character: {repeatedCharErrorInput} ... {result}', color)
    match result[0]:
        case 'success':
            repeatedCharSuccesses += 1
        case 'skipped':
            repeatedCharSkips += 1
        case 'partial success':
            repeatedCharPartialSuccesses += 1

    unicharErrorInput = typo.StrErrer(line).unichar().result
    result = run(unicharErrorInput, line)
    color = get_color(result[0])
    cprint(f'unicharacter: {unicharErrorInput} ... {result}', color)
    match result[0]:
        case 'success':
            unicharSuccesses += 1
        case 'skipped':
            unicharSkips += 1
        case 'partial success':
            unicharPartialSuccesses += 1

    print()

with open('input.txt', 'r') as file:
    testCommandLines = file.read().strip().split("\n")

executable = '../target/release/cli_assist'

for commandLine in testCommandLines:
    test(commandLine)

print('===========================================================\n')

print(f'Character swap: {charSwapSuccesses}/{charSwapPartialSuccesses}/{len(testCommandLines)-charSwapSkips}')
print(f'Missing character: {missingCharSuccesses}/{missingCharPartialSuccesses}/{len(testCommandLines)-missingCharSkips}')
print(f'Extra character: {extraCharSuccesses}/{extraCharPartialSuccesses}/{len(testCommandLines)-extraCharSkips}')
print(f'Nearby character: {nearbyCharSuccesses}/{nearbyCharPartialSuccesses}/{len(testCommandLines)-nearbyCharSkips}')
print(f'Skipped space: {skippedSpaceSuccesses}/{skippedSpacePartialSuccesses}/{len(testCommandLines)-skippedSpaceSkips}')
print(f'Random space: {randomSpaceSuccesses}/{randomSpacePartialSuccesses}/{len(testCommandLines)-randomSpaceSkips}')
print(f'Repeated character: {repeatedCharSuccesses}/{repeatedCharPartialSuccesses}/{len(testCommandLines)-repeatedCharSkips}')
print(f'Unicharacter: {unicharSuccesses}/{unicharPartialSuccesses}/{len(testCommandLines)-unicharSkips}')

print('\n===========================================================\n')

print('TOTAL')
print(f'Successes: {success}')
print(f'Partial successes: {partialCorrection}')
print(f'No corrections: {noCorrection}')
print(f'Skips: {skipped}')
print(f'Errors: {error}')

