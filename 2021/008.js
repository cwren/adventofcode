#!javascript

var tools = {
    parseLine: function(s) {
        [p, o] = s.split(" | ");
        patterns = p.split(" ", 10);
        outputs = o.split(" ", 4);
        return [patterns, outputs];
    },

    ones: function(digits) {
        return digits.filter(d => d.length == 2);
    },

    fours: function(digits) {
        return digits.filter(d => d.length == 4);
    },

    sevens: function(digits) {
        return digits.filter(d => d.length == 3);
    },

    eights: function(digits) {
        return digits.filter(d => d.length == 7);
    },

    norm: function(a) {
        return a.split('').sort().join('');
    },
    
    subtract: function(a, b) {
        var A = new Set(a.split(''));
        var B = new Set(b.split(''));
        var C = new Set([...A].filter(x => !B.has(x)));
        return Array.from(C).sort().join('');
    },
    
    decode: function(s) {
        [patterns, outputs] = tools.parseLine(s);
        var signals = {
            2: [],
            3: [],
            4: [],
            5: [],
            6: [],
            7: [],
        };
        patterns.forEach(p => signals[p.length].push(tools.norm(p)));
        
        var digit = new Map();
        digit.set(1, signals[2][0]);
        digit.set(7, signals[3][0]);
        digit.set(4, signals[4][0]);
        digit.set(8, signals[7][0]);
        
        // -0-
        // | |
        // 1 2
        // | |
        // -3-
        // | |
        // 4 5
        // | |
        // -6-
        var light = {}; 
        
        // one light diff between 1 and 7
        light[0] = tools.subtract(digit.get(7), digit.get(1));

        // find 3 (3 lights vs four lights for 2 and 5)
        signals[5].forEach(signal => {
            if (tools.subtract(signal, digit.get(1)).length == 3) {
                digit.set(3, signal);
            }
        });

        // one light diff between 4 and 3
        light[1] = tools.subtract(digit.get(4), digit.get(3));

        // find 2 and 5
        signals[5].forEach(signal => {
            if (signal != digit.get(3)) {
                var l = tools.subtract(signal, digit.get(3));
                if (l == light[1]) {
                    digit.set(5, signal);
                } else {
                    digit.set(2, signal);
                    light[4] = l;
                }
            }
        });

        // find light 2
        light[2] = tools.subtract(digit.get(1), digit.get(5));

        
        // find 0, 6, and 9
        signals[6].forEach(signal => {
            var l = tools.subtract(digit.get(8), signal);
            switch (l) {
            case light[2]:
                digit.set(6, signal);
                break;
            case light[4]:
                digit.set(9, signal);
                break;
            default:
                digit.set(0, signal);
                light[3] = l;
                break;
            }
        });

        var decoder = new Map();
        digit.forEach((v, k) => decoder.set(v, k));
        return (decoder.get(tools.norm(outputs[0])) * 1000 +
                decoder.get(tools.norm(outputs[1])) * 100 +
                decoder.get(tools.norm(outputs[2])) * 10 +
                decoder.get(tools.norm(outputs[3])) * 1);
    },
};    
module.exports = tools;

function main() {
    var lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('008.txt')
    });
    var total_a = 0;
    var total_b = 0;
    lineReader.on('line', (line) => {
        [p, o] = tools.parseLine(line);
        total_a += (tools.ones(o).length +
                    tools.fours(o).length +
                    tools.sevens(o).length +
                    tools.eights(o).length);
        total_b += tools.decode(line);
    });

    lineReader.on('close', () => {
        console.log(`a: ${total_a} `);
        console.log(`b: ${total_b} `);
    });
};

if (require.main === module) {
    main();
}
