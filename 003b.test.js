const tools = require('./003b.js');

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

test('test bitcount', () => {
    var input = [...TEST_INPUT];
    var t = []
    input.forEach(s => tools.parseline(t, s))
    b = tools.countbits(12, t);
    expect(b).toEqual(EXPECTED_B);
});

test('filter for e', () => {
    var input = [...TEST_INPUT];
    var t = []
    input.forEach(s => tools.parseline(t, s))
    e = tools.filter_for_e(12, t);
    expect(e).toBe(EXPECTED_E);
});

test('filter for g', () => {
    var input = [...TEST_INPUT];
    var t = []
    input.forEach(s => tools.parseline(t, s))
    e = tools.filter_for_g(12, t);
    expect(e).toBe(EXPECTED_G);
});
