#!javascript

// e: 010111100100
// g: 101000011011
// 1508 * 2587 = 3901196 

var lineReader = require('readline').createInterface({
  input: require('fs').createReadStream('003.txt')
});

l = [];
n = 0;

lineReader.on('line', (line) => {
    n = Math.max(n, line.length)
    d = parseInt(line, 2);
    l.push(d)
});

function countbits(n, l) {
    var b = new Array(n).fill(0);
    for (let i = 0; i < b.length; i++) {
        l.forEach(number => b[i] += ((number >> i) & 0x1))
    }
    b.forEach((v, i) => b[i] = v > (l.length / 2) ? 1 : 0)
    return b
};

lineReader.on('close', () => {
    b = countbits(n, l)
    e = 0;
    g = 0;
    console.log(`${b}`);
    for (let i = 0; i < b.length; i++) {
        if (b[i] == 1) {
            e |= 0x1 << i
        } else {
            g |= 0x1 << i
        }
    }
    
    console.log(`e: ${(e >>> 0).toString(2)}`);
    console.log(`g: ${(g >>> 0).toString(2)}`);
    console.log(`${e} * ${g} = ${e*g} `);
});
