const tools = require('./014.js');
TEST_INPUT = [
    'NNCB',
    '',
    'CH -> B',
    'HH -> N',
    'CB -> H',
    'NH -> C',
    'HB -> C',
    'HC -> B',
    'HN -> C',
    'NN -> C',
    'BH -> H',
    'NC -> B',
    'NB -> B',
    'BN -> B',
    'BB -> N',
    'BC -> B',
    'CC -> N',
    'CN -> C',
];
EXPECTED_SEQUENCES = new Map([
    // NNCB
    [0, new Map([['B',  1], ['C',  1], ['H', 0], ['N',  2]])], 
    // NCNBCHB
    [1, new Map([['B',  2], ['C',  2], ['H', 1], ['N',  2]])], 
    //  NBCCNBBBCBHCB
    [2, new Map([['B',  6], ['C',  4], ['H', 1], ['N',  2]])], 
    // NBBBCNCCNBBNBNBBCHBHHBCHB
    [3, new Map([['B', 11], ['C',  5], ['H', 4], ['N',  5]])], 
    // NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
    [4, new Map([['B', 23], ['C', 10], ['H', 5], ['N', 11]])], 
]);
EXPECTED_LENGTH_AT_5 = 97;
EXPECTED_LENGTH_AT_10 = 3073;
EXPECTED_SCORE_AT_10 = 1588;
EXPECTED_SCORE_AT_40 = 2188189693529;

test('test parser', () => {
    var seq = new tools.Sequence();
    for (line of TEST_INPUT) {
        seq.parse(line);
    }
    seq.init();
    expect(seq.bases).toEqual(EXPECTED_SEQUENCES.get(0));
});

test('test grow', () => {
    var seq = new tools.Sequence();
    for (line of TEST_INPUT) {
        seq.parse(line);
    }
    seq.init();
    for (var t = 0; t < 5; t++) {
        expect(seq.bases).toEqual(EXPECTED_SEQUENCES.get(t));
        seq.grow();
    }
});

test('test length', () => {
    var seq = new tools.Sequence();
    for (line of TEST_INPUT) {
        seq.parse(line);
    }
    seq.init();
    for (var t = 0; t < 11; t++) {
        if (t == 5) {
            expect(seq.length).toBe(EXPECTED_LENGTH_AT_5);
        }
        if (t == 10) {
            expect(seq.length).toBe(EXPECTED_LENGTH_AT_10);
        }
        seq.grow();
    }
});

test('test score 10', () => {
    var seq = new tools.Sequence();
    for (line of TEST_INPUT) {
        seq.parse(line);
    }
    seq.init();
    for (var t = 0; t < 10; t++) {
        seq.grow();
    }
    expect(seq.score).toBe(EXPECTED_SCORE_AT_10);
});

test('test score 40', () => {
    var seq = new tools.Sequence();
    for (line of TEST_INPUT) {
        seq.parse(line);
    }
    seq.init();
    for (var t = 0; t < 40; t++) {
        seq.grow();
    }
    expect(seq.score).toBe(EXPECTED_SCORE_AT_40);
});


