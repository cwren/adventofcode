const tools = require('./012.js');
SYSTEM_A = [
    'start-A',
    'start-b',
    'A-c',
    'A-b',
    'b-d',
    'A-end',
    'b-end',
];
PATHS_A = 10;
MULTIVISIT_PATHS_A = 36;
MAP_A = new Map([
    ['start', ['A', 'b']],
    ['A', ['b', 'c', 'end', 'start']],
    ['b', ['A', 'd', 'end', 'start']],
    ['c', ['A']],
    ['d', ['b']],
]);
                
SYSTEM_B = [
    'dc-end',
    'HN-start',
    'start-kj',
    'dc-start',
    'dc-HN',
    'LN-dc',
    'HN-end',
    'kj-sa',
    'kj-HN',
    'kj-dc',
];
PATHS_B = 19;
MULTIVISIT_PATHS_B = 103;

SYSTEM_C = [
    'fs-end',
    'he-DX',
    'fs-he',
    'start-DX',
    'pj-DX',
    'end-zg',
    'zg-sl',
    'zg-pj',
    'pj-he',
    'RW-he',
    'fs-DX',
    'pj-RW',
    'zg-RW',
    'start-pj',
    'he-WI',
    'zg-he',
    'pj-fs',
    'start-RW',
];
PATHS_C = 226;
MULTIVISIT_PATHS_C = 3509;

test('test parser', () => {
    var caves = new tools.Caves();
    for (line of SYSTEM_A) {
        caves.parse(line);
    }
    for(cave of MAP_A.keys()) {
        for (connection of MAP_A.get(cave)){
            expect(caves.get(cave)).toContain (connection);
            expect(caves.get(connection)).toContain(cave);
        }
    }
});

test('test numPaths A', () => {
    var caves = tools.Caves.fromMap(MAP_A);
    expect(caves.numPaths).toBe(PATHS_A);
});

test('test numPaths B', () => {
    var caves = new tools.Caves();
    for (line of SYSTEM_B) {
        caves.parse(line);
    }
    expect(caves.numPaths).toBe(PATHS_B);
});

test('test numPaths C', () => {
    var caves = new tools.Caves();
    for (line of SYSTEM_C) {
        caves.parse(line);
    }
    expect(caves.numPaths).toBe(PATHS_C);
});

test('test fromMap', () => {
    var caves = tools.Caves.fromMap(MAP_A);
    for(cave of caves.keys()) {
        expect(caves.get(cave)).toEqual(MAP_A.get(cave));
    }
    expect(Array.from(caves.keys())).toEqual(Array.from(MAP_A.keys()));
});

test('test multiPaths.length A', () => {
    var caves = tools.Caves.fromMap(MAP_A);
    expect(caves.multiPaths.length).toBe(MULTIVISIT_PATHS_A);
});

test('test multiPaths.length B', () => {
    var caves = new tools.Caves();
    for (line of SYSTEM_B) {
        caves.parse(line);
    }
    expect(caves.multiPaths.length).toBe(MULTIVISIT_PATHS_B);
});

test('test multiPaths.length C', () => {
    var caves = new tools.Caves();
    for (line of SYSTEM_C) {
        caves.parse(line);
    }
    expect(caves.multiPaths.length).toBe(MULTIVISIT_PATHS_C);
});

MULTIPATHS_A = [
    'start,A,b,A,b,A,c,A,end',
    'start,A,b,A,b,A,end',
    'start,A,b,A,b,end',
    'start,A,b,A,c,A,b,A,end',
    'start,A,b,A,c,A,b,end',
    'start,A,b,A,c,A,c,A,end',
    'start,A,b,A,c,A,end',
    'start,A,b,A,end',
    'start,A,b,d,b,A,c,A,end',
    'start,A,b,d,b,A,end',
    'start,A,b,d,b,end',
    'start,A,b,end',
    'start,A,c,A,b,A,b,A,end',
    'start,A,c,A,b,A,b,end',
    'start,A,c,A,b,A,c,A,end',
    'start,A,c,A,b,A,end',
    'start,A,c,A,b,d,b,A,end',
    'start,A,c,A,b,d,b,end',
    'start,A,c,A,b,end',
    'start,A,c,A,c,A,b,A,end',
    'start,A,c,A,c,A,b,end',
    'start,A,c,A,c,A,end',
    'start,A,c,A,end',
    'start,A,end',
    'start,b,A,b,A,c,A,end',
    'start,b,A,b,A,end',
    'start,b,A,b,end',
    'start,b,A,c,A,b,A,end',
    'start,b,A,c,A,b,end',
    'start,b,A,c,A,c,A,end',
    'start,b,A,c,A,end',
    'start,b,A,end',
    'start,b,d,b,A,c,A,end',
    'start,b,d,b,A,end',
    'start,b,d,b,end',
    'start,b,end',
];
test('test multiPaths A', () => {
    var caves = tools.Caves.fromMap(MAP_A);
    var paths = caves.multiPaths;
    for(path of MULTIPATHS_A) {
        expect(paths).toContain(path);
    }
    for(path of paths) {
        expect(MULTIPATHS_A).toContain(path);
    }
});

