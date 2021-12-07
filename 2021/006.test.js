const tools = require('./006.js');
TEST_INPUT = '3,4,3,1,2';
EXPECTED_DAY0 = [0, 1, 1, 2, 1, 0, 0, 0, 0]; // [3, 4, 3, 1, 2]
EXPECTED_DAY1 = [1, 1, 2, 1, 0, 0, 0, 0, 0]; // [2, 3, 2, 0, 1]
EXPECTED_DAY2 = [1, 2, 1, 0, 0, 0, 1, 0, 1]; // [1, 2, 1, 6, 0, 8]
EXPECTED_DAY3 = [2, 1, 0, 0, 0, 1, 1, 1, 1]; // [0, 1, 0, 5, 6, 7, 8]
EXPECTED_DAY4 = [1, 0, 0, 0, 1, 1, 3, 1, 2]; // [6, 0, 6, 4, 5, 6, 7, 8, 8]

test('test fish reader', () => {
    expect(tools.parseLine(TEST_INPUT)).toEqual(EXPECTED_DAY0);
});

test('test growth', () => {
    expect(tools.tick(EXPECTED_DAY0)).toEqual(EXPECTED_DAY1);
});

test('test spawn', () => {
    expect(tools.tick(EXPECTED_DAY1)).toEqual(EXPECTED_DAY2);
    expect(tools.tick(EXPECTED_DAY2)).toEqual(EXPECTED_DAY3);
    expect(tools.tick(EXPECTED_DAY3)).toEqual(EXPECTED_DAY4);
});

test('test generation', () => {
    var school = EXPECTED_DAY0;
    for (var t = 0; t < 18; t ++) {
        school = tools.tick(school);
    }
    expect(tools.census(school)).toBe(26);
});

test('test grey ooze', () => {
    var school = EXPECTED_DAY0;
    for (var t = 0; t < 256; t ++) {
        school = tools.tick(school);
    }
    expect(tools.census(school)).toBe(26984457539);
});

