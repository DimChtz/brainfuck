function generateBFCode(text) {
    
    var bf = "";

    var base = [];

    // Create the base cells
    for ( var i = 0; i < text.length; ++i ) {
        base[i] = Math.floor(text.charCodeAt(i) / 10) * 10;
    }

    // Remove duplicate base values
    base = base.filter(function(item, pos, self) {
        return self.indexOf(item) == pos;
    })

    // Write base to the tape
    bf += Array(11).join("+") + "[";

    for ( var i = 0; i < base.length; ++i ) {
        bf += ">" + Array( Math.floor(base[i] / 10) + 1 ).join("+");
    }

    bf += Array(base.length + 1).join("<") + "-]";

    // Print all characters
    var lastPos = 0;
    for ( var i = 0; i < text.length; ++i ) {

        var minDist = Math.abs( text.charCodeAt(i) - base[0] );
        var minBase = base[0];
        var minPos = 1;

        for ( var j = 0; j < base.length; ++j ) {

            if ( Math.abs( text.charCodeAt(i) - base[j] ) < minDist ) {

                minDist = Math.abs( text.charCodeAt(i) - base[j] );
                minBase = base[j];
                minPos = j + 1;

            }

        }

        var op = "+";
        var dir = ">";

        if ( text.charCodeAt(i) < minBase ) op = "-";
        if ( minPos < lastPos ) dir = "<";

        bf += Array( Math.abs(minPos - lastPos) + 1 ).join(dir);
        bf += Array( Math.abs(text.charCodeAt(i) - minBase) + 1 ).join(op);
        bf += ".";

        lastPos = minPos;
        base[minPos - 1] = text.charCodeAt(i);
  
    }

    console.log(bf);

    return bf;

}
