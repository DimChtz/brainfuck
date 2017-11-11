import math

def generateBFCode(text):
    
    result = ""
    lastchar = '\0'
    factor = 0
    remain = 0
    direction = '+'

    for c in text:

        if ord(c) > ord(lastchar):
            factor = math.floor(math.sqrt(ord(c) - ord(lastchar)))
            remain = (ord(c) - ord(lastchar)) - factor * factor
            direction = '+'
        else:
            factor = math.floor(math.sqrt(ord(lastchar) - ord(c)))
            remain = (ord(lastchar) - ord(c)) - factor * factor
            direction = '-'

        result += '+' * factor + '[>' + direction * factor + '<-]>' + direction * remain + '.<'

        lastchar = c

    return result
