const tools = require('./017.js');

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
