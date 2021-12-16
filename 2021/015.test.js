const tools = require('./015.js');
TEST_INPUT = [
    '1163751742',
    '1381373672',
    '2136511328',
    '3694931569',
    '7463417111',
    '1319128137',
    '1359912421',
    '3125421639',
    '1293138521',
    '2311944581',
];
EXPECTED_FIELD = [
    [1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
    [1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
    [2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
    [3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
    [7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
    [1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
    [1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
    [3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
    [1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
    [2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
];
EXPECTED_RISK = 40;
EXPECTED_RISK_5 = 315;

test('test parser', () => {
    var cave = new tools.Cave();
    for (line of TEST_INPUT) {
        cave.parse(line);
    }
    cave.setMultiplier(1);
    expect(cave.r).toEqual(EXPECTED_FIELD);
});

test('test risk wrap 1', () => {
    var cave = new tools.Cave();
    for (line of TEST_INPUT) {
        cave.parse(line);
    }
    cave.setMultiplier(1);

    for(let i = 0; i < cave.w; i++) {
        for(let j = 0; j < cave.h; j++) {
            expect(cave.getRisk([i, j])).toBe(EXPECTED_FIELD[j][i]);
        }
    }
});

test('test risk wrap 2', () => {
    var cave = new tools.Cave();
    for (line of TEST_INPUT) {
        cave.parse(line);
    }
    cave.setMultiplier(2);

    expect(cave.getRisk([10,  10])).toBe(3);
    expect(cave.getRisk([10,  0])).toBe(2);
    expect(cave.getRisk([0,  10])).toBe(2);

    expect(cave.getRisk([9,   9])).toBe(1);
    expect(cave.getRisk([19, 19])).toBe(3);
});
     
test('test risk wrap 5', () => {
    var cave = new tools.Cave();
    for (line of TEST_INPUT) {
        cave.parse(line);
    }
    cave.setMultiplier(5);

    expect(cave.getRisk([49, 0])).toBe(6);
    expect(cave.getRisk([47, 0])).toBe(2);
    expect(cave.getRisk([0, 49])).toBe(6);
    expect(cave.getRisk([49,49])).toBe(9);
});
     
test('test neighbors', () => {
    var cave = new tools.Cave();
    for (line of TEST_INPUT) {
        cave.parse(line);
    }
    cave.setMultiplier(1);
    expect(cave.w).toBe(10);
    expect(cave.h).toBe(10);
    expect([...cave.neighbors([5, 5])].length).toBe(4);
    expect([...cave.neighbors([0, 0])].length).toBe(2);
    expect([...cave.neighbors([9, 9])].length).toBe(2);
    expect([...cave.neighbors([0, 5])].length).toBe(3);
    expect([...cave.neighbors([5, 0])].length).toBe(3);
});

test('test score', () => {
    var cave = new tools.Cave();
    for (line of TEST_INPUT) {
        cave.parse(line);
    }
    cave.setMultiplier(1);
    expect(cave.risk).toBe(EXPECTED_RISK);
});

test('test score x 5', () => {
    var cave = new tools.Cave();
    for (line of TEST_INPUT) {
        cave.parse(line);
    }
    cave.setMultiplier(5);
    expect(cave.risk).toBe(EXPECTED_RISK_5);
});


