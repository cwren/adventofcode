#!javascript

var tools = {
    parseline: function(a, s) {
    a.push(parseInt(s, 2))
    },
    countbits: function(n, l) {
        var b = new Array(n).fill(0);
        for (let i = 0; i < b.length; i++) {
            l.forEach(number => b[i] += ((number >> i) & 0x1))
        }
        b.forEach((v, i) => b[i] = v >= (l.length / 2) ? 1 : 0)
        return b
    },
    filter_for_e: function(bits, field) {
        var i = bits - 1;
        var c = [...field];
        while(c.length > 1) {
            b = tools.countbits(bits, c);
            c = c.filter(v => ((v >> i) & 0x1) === b[i]);
            i -= 1;
        }
        return c[0];
    },
    filter_for_g: function(bits, field) {
        var i = bits - 1;
        var c = [...field];
        while(c.length > 1) {
            b = tools.countbits(bits, c);
            c = c.filter(v => ((v >> i) & 0x1) !== b[i]);
            i -= 1;
        }
        return c[0];
    }
};    
module.exports = tools;

function main() {
    var n = 0
    var l = []

    var lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('003.txt')
    });
    
    lineReader.on('line', (line) => {
        n = Math.max(n, line.length)
    tools.parseline(l, line)
    });
    
    lineReader.on('close', () => {
        e = tools.filter_for_e(n, l)
        g = tools.filter_for_g(n, l)
        
        console.log(`e: ${(e >>> 0).toString(2)}`);
        console.log(`g: ${(g >>> 0).toString(2)}`);
        console.log(`${e} * ${g} = ${e*g} `);
    });
};

if (require.main === module) {
  main();
}
// too low: v0
// e: 10100
// g: 101011110001
// 20 * 2801 = 56020

// too high: v >= (l.length / 2)
// e: 11100010100
// g: 101011110001
// 1812 * 2801 = 5075412 
