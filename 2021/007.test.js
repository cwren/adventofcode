const tools = require('./007.js');
TEST_INPUT = '16,1,2,0,4,2,7,1,2,14';
EXPECTED_POS = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
EXPECTED_FUEL_1 = 41;
EXPECTED_FUEL_2 = 37;
EXPECTED_FUEL_3 = 39;
EXPECTED_FUEL_10 = 71;

test('test parser', () => {
    expect(tools.parseLine(TEST_INPUT)).toEqual(EXPECTED_POS);
});

test('test linear', () => {
    expect(tools.linear(EXPECTED_POS,  1)).toBe(41);
    expect(tools.linear(EXPECTED_POS,  2)).toBe(37);
    expect(tools.linear(EXPECTED_POS,  3)).toBe(39);
    expect(tools.linear(EXPECTED_POS, 10)).toBe(71);
});

test('test quad', () => {
    expect(tools.quad(EXPECTED_POS,  2)).toBe(206);
    expect(tools.quad(EXPECTED_POS,  5)).toBe(168);
});

test('test linear min', () => {
    expect(tools.min(EXPECTED_POS, tools.linear)).toBe(2);
});

test('test quad min', () => {
    expect(tools.min(EXPECTED_POS, tools.quad)).toBe(5);
});

