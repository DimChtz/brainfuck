def generateBFCode(text)

    bf = ""
    lastchar = 0

    text.each_byte { |c|

        factor = 0
        remain = 0
        direction = '+'

        if c > lastchar
            factor = (Math.sqrt(c - lastchar)).floor
            remain = (c - lastchar) - (factor * factor)
            direction = '+'
        else
            factor = (Math.sqrt(lastchar - c)).floor
            remain = (lastchar - c) - (factor * factor)
            direction = '-'
        end

        bf += ('+' * factor) + '[>' + (direction * factor) + '<-]>' + (direction * remain) + '.<'

        lastchar = c

    }

    return bf

end
