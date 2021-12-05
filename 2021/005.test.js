const tools = require('./005.js');
TEST_INPUT = [
    '491,392 -> 34,392',
    '337,52 -> 485,52',
    '256,605 -> 256,959',
    '889,142 -> 153,878',
    '189,59 -> 512,382',
    '399,193 -> 598,193',
    '578,370 -> 795,153',
    '79,450 -> 569,450',
    '565,444 -> 270,149',
    '39,28 -> 39,846'
];
EXPECTED_LINES = [
    [[491, 392], [34, 392]],
    [[337, 52], [485, 52]],
    [[256, 605], [256, 959]],
    [[889, 142], [153, 878]],
    [[189, 59], [512, 382]],
    [[399, 193], [598, 193]],
    [[578, 370], [795, 153]],
    [[79, 450], [569, 450]],
    [[565, 444], [270, 149]],
    [[39, 28], [39, 846]],
];
EXPECTED_HORIZONTAL = [
    [[491, 392], [34, 392]],
    [[337, 52], [485, 52]],
    [[399, 193], [598, 193]],
    [[79, 450], [569, 450]],
];
EXPECTED_VERTICAL = [
    [[256, 605], [256, 959]],
    [[39, 28], [39, 846]],
];
EXPECTED_DIAGONAL = [
    [[889, 142], [153, 878]],
    [[189, 59], [512, 382]],
    [[578, 370], [795, 153]],
    [[565, 444], [270, 149]],
];

test('test line reader', () => {
    lines = [];
    TEST_INPUT.forEach(l => lines.push(tools.parseLine(l)));
    expect(lines).toEqual(EXPECTED_LINES);
});

test('test h filter', () => {
    r = EXPECTED_LINES.filter(l => tools.isHorizontal(l));
    expect(r).toEqual(EXPECTED_HORIZONTAL);
});

test('test v filter', () => {
    r = EXPECTED_LINES.filter(l => tools.isVertical(l));
    expect(r).toEqual(EXPECTED_VERTICAL);
});

test('test d filter', () => {
    r = EXPECTED_LINES.filter(l => tools.isDiagonal(l));
    expect(r).toEqual(EXPECTED_DIAGONAL);
});

test('test no intersection', () => {
    sfm = new tools.SeaFloorMap();
    sfm.draw([[79, 450], [569, 450]]);
    sfm.draw([[39, 28], [39, 846]]);
    expect(sfm.num_intersections).toBe(0);
});

test('test 1 intersection', () => {
    sfm = new tools.SeaFloorMap();
    sfm.draw([[19, 450], [569, 450]]);
    sfm.draw([[39, 28], [39, 846]]);
    expect(sfm.num_intersections).toBe(1);
});

test('test 2 intersection', () => {
    sfm = new tools.SeaFloorMap();
    sfm.draw([[19, 450], [569, 450]]);
    sfm.draw([[39, 28], [39, 846]]);
    sfm.draw([[329, 450], [329, 450]]);
    expect(sfm.num_intersections).toBe(2);
});

test('test ++ diagonals', () => {
    sfm = new tools.SeaFloorMap();
    sfm.draw([[100, 150], [200, 150]], hv_only=false);
    sfm.draw([[100, 100], [200, 200]], hv_only=false);
    expect(sfm.num_intersections).toBe(1);
});

test('test ++ diagonals, exact', () => {
    sfm = new tools.SeaFloorMap();
    sfm.draw([[150, 100], [150, 200]], hv_only=false);
    sfm.draw([[100, 150], [200, 150]], hv_only=false);
    sfm.draw([[100, 100], [200, 200]], hv_only=false);
    expect(sfm.num_intersections).toBe(1);
});

test('test +- diagonals', () => {
    sfm = new tools.SeaFloorMap();
    sfm.draw([[100, 150], [200, 150]], hv_only=false);
    sfm.draw([[100, 200], [200, 100]], hv_only=false);
    expect(sfm.num_intersections).toBe(1);
});

test('test -+ diagonals', () => {
    sfm = new tools.SeaFloorMap();
    sfm.draw([[100, 150], [200, 150]], hv_only=false);
    sfm.draw([[200, 100], [100, 200]], hv_only=false);
    expect(sfm.num_intersections).toBe(1);
});

test('test -- diagonals', () => {
    sfm = new tools.SeaFloorMap();
    sfm.draw([[100, 150], [200, 150]], hv_only=false);
    sfm.draw([[200, 200], [100, 100]], hv_only=false);
    expect(sfm.num_intersections).toBe(1);
});

