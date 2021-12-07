#!javascript

var tools = {
    parseLine: function(s) {
        return s.split(",").map(item => parseInt(item))
    },

    linear: function(x, a) {
        return x.reduce((f, p) => f += Math.abs(p - a), 0);
    },

    quad: function(x, a) {
        return x.reduce((f, p) => {
            let d = Math.abs(p - a);
            return f += d * (d + 1) / 2;
        }, 0);
    },

    min: function(x, measure) {
        var p = 0;
        var f = Number.MAX_VALUE;
        for(var i = Math.min(...x); i < Math.max(...x); i++) {
            var b = measure(x, i);
            if (b < f) {
                p = i;
                f = b;
            }
        }
        return (p);
    },
};    
module.exports = tools;

function main(measure) {
    var lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('007.txt')
    });
    var x;
    lineReader.on('line', (line) => {
        x = tools.parseLine(line);
    });

    lineReader.on('close', () => {
        var a = tools.min(x, measure);
        console.log(`${measure(x, a)}`);
    });
};

if (require.main === module) {
    var arg = process.argv[process.argv.length - 1];
    var measure = arg == "quad" ? tools.quad : tools.linear;
    main(measure);
}
