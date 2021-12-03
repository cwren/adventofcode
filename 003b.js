#!javascript

function parseline(a, s) {
    a.push(parseInt(s, 2))
};

TEST_INPUT = [
    '001001100101',
    '010100011100',
    '100000110001',
    '001111110101',
    '100010110101',
    '111010100100',
    '011011000110',
    '100000011101',
    '011001100111',
    '000001011110',
    '000010100011',
    '110100111110',
    '001101100101',
    '011011011101',
    '010000011010',
    '011100100100',
    '001111000011',
    '100111000111',
    '100111100011',
    '101100011011'
];
EXPECTED_B = [1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0];
EXPECTED_E = 0b011011011101;
EXPECTED_G = 0b110100111110;

function countbits(n, l) {
    var b = new Array(n).fill(0);
    for (let i = 0; i < b.length; i++) {
        l.forEach(number => b[i] += ((number >> i) & 0x1))
    }
    b.forEach((v, i) => b[i] = v >= (l.length / 2) ? 1 : 0)
    return b
};


function test_b() {
    var input = [...TEST_INPUT];
    var t = []
    input.forEach(s => parseline(t, s))
    console.log(`${t}`)
    b = countbits(12, t);
    console.assert(b.every((e, i) => e === EXPECTED_B[i]), "wrong b")
};
test_b();

function filter_for_e(bits, field) {
    var i = bits - 1;
    var c = [...field];
    while(c.length > 1) {
        b = countbits(bits, c);
        c = c.filter(v => ((v >> i) & 0x1) === b[i]);
        i -= 1;
    }
    return c[0];
};

function test_e() {
    var input = [...TEST_INPUT];
    var t = []
    input.forEach(s => parseline(t, s))
    e = filter_for_e(12, t);
    console.log(`e': ${(e >>> 0).toString(2)}`);
    console.assert(e === EXPECTED_E , "wrong e")
};
test_e();


function filter_for_g(bits, field) {
    var i = bits - 1;
    var c = [...field];
    while(c.length > 1) {
        b = countbits(bits, c);
        c = c.filter(v => ((v >> i) & 0x1) !== b[i]);
        i -= 1;
    }
    return c[0];
};

function test_g() {
    var input = [...TEST_INPUT];
    var t = []
    input.forEach(s => parseline(t, s))
    g = filter_for_g(12, t);
    console.log(`g': ${(e >>> 0).toString(2)}`);
    console.assert(g === EXPECTED_G , "wrong g")
};
test_g();


var lineReader = require('readline').createInterface({
  input: require('fs').createReadStream('003.txt')
});


var n = 0
var l = []
lineReader.on('line', (line) => {
    n = Math.max(n, line.length)
    parseline(l, line)
});


lineReader.on('close', () => {
    e = filter_for_e(n, l)
    g = filter_for_g(n, l)

    console.log(`e: ${(e >>> 0).toString(2)}`);
    console.log(`g: ${(g >>> 0).toString(2)}`);
    console.log(`${e} * ${g} = ${e*g} `);
});

// too low: v0
// e: 10100
// g: 101011110001
// 20 * 2801 = 56020

// too high: v >= (l.length / 2)
// e: 11100010100
// g: 101011110001
// 1812 * 2801 = 5075412 
