const tools = require('./017.js');

MAGNITUDES = [
    [' [[1,2],[[3,4],5]]', 143],
    [' [[[[0,7],4],[[7,8],[6,0]]],[8,1]]', 1384],
    [' [[[[1,1],[2,2]],[3,3]],[4,4]]', 445],
    [' [[[[3,0],[5,3]],[4,4]],[5,5]]', 791],
    [' [[[[5,0],[7,4]],[5,5]],[6,6]]', 1137],
    [' [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]', 348],
];

SAMPLE_HOMEWORK = [
    '[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]',
    '[[[5,[2,8]],4],[5,[[9,9],0]]]',
    '[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]',
    '[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]',
    '[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]',
    '[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]',
    '[[[[5,4],[7,7]],8],[[8,3],8]]',
    '[[9,3],[[9,9],[6,[4,9]]]]',
    '[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]',
    '[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]',
];
SAMPLE_SOLUTION = '[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]';
SAMPLE_MAGNITUDE = 4140;

TEST_INPUT = 'target area: x=20..30, y=-10..-5';
EXPECTED_TARGET = new tools.Target(20, 30, -10, -5);

test('test parser', () => {
    var target = tools.Target.fromString(TEST_INPUT);
    expect(target).toEqual(EXPECTED_TARGET);
});

test('test highest', () => {
    var target = tools.Target.fromString(TEST_INPUT);
    expect(target.highest()).toBe(45);
});

test('test miss', () => {
    expect(EXPECTED_TARGET.hit( 0,   0)).toBeFalsy();
    expect(EXPECTED_TARGET.hit(40,  -8)).toBeFalsy();
    expect(EXPECTED_TARGET.hit(10,  -8)).toBeFalsy();
    expect(EXPECTED_TARGET.hit(25, -11)).toBeFalsy();
    expect(EXPECTED_TARGET.hit(25,  -1)).toBeFalsy();
    expect(EXPECTED_TARGET.hit(25,   1)).toBeFalsy();
});

test('test hit', () => {
    expect(EXPECTED_TARGET.hit(25,  -8)).toBeTruthy();
    expect(EXPECTED_TARGET.hit(20,  -8)).toBeTruthy();
    expect(EXPECTED_TARGET.hit(30,  -8)).toBeTruthy();
    expect(EXPECTED_TARGET.hit(25, -10)).toBeTruthy();
    expect(EXPECTED_TARGET.hit(25,  -6)).toBeTruthy();
    expect(EXPECTED_TARGET.hit(25,  -5)).toBeTruthy();
    expect(EXPECTED_TARGET.hit(28,  -7)).toBeTruthy();
});

test('test simulate', () => {
    expect(EXPECTED_TARGET.simulate( 7,  2)[0]).toBeTruthy();
    expect(EXPECTED_TARGET.simulate( 6,  3)[0]).toBeTruthy();
    expect(EXPECTED_TARGET.simulate( 9,  0)[0]).toBeTruthy();
    expect(EXPECTED_TARGET.simulate(17, -4)[0]).toBeFalsy();
    expect(EXPECTED_TARGET.simulate( 1,  0)[0]).toBeFalsy();
});

test('test total', () => {
    expect(EXPECTED_TARGET.numHits()).toBe(112);
});
