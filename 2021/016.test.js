const tools = require('./016.js');

test('test bits', () => {
    var bits = new tools.BITS();
    expect(bits.stringToBits('D2FE28')).toEqual([
        1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0
    ]);
});

test('test literal parser', () => {
    var bits = new tools.BITS();
    var packet = bits.parse('D2FE28'); // 110 100 1 0111 1 1110 0 0101 000
    expect(packet.version).toBe(6);
    expect(packet.type).toBe(4);
    expect(packet.value).toBe(2021n);
});

// 1101001111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111101111
test('test bignum', () => {
    var bits = new tools.BITS();
    var packet = bits.parse('D3FFFFFFFFFFFFFFFFFFFFFFFBC');
    expect(packet.version).toBe(6);
    expect(packet.type).toBe(4);
    expect(packet.value).toBe(1208925819614629174706175n);
});

test('test length sub parser', () => {
    var bits = new tools.BITS();
    var packet = bits.parse('38006F45291200');
    expect(packet.version).toBe(1);
    expect(packet.type).toBe(6);
    var subs = [[6, 4, 10n], [2, 4, 20n]];
    for (let i = 0; i < 2; i++) {
        expect(packet.sub[i].version).toBe(subs[i][0]);
        expect(packet.sub[i].type   ).toBe(subs[i][1]);
        expect(packet.sub[i].value  ).toBe(subs[i][2]);
    }
});

test('test count sub parser', () => {
    var bits = new tools.BITS();
    var packet = bits.parse('EE00D40C823060');
    expect(packet.version).toBe(7);
    expect(packet.type).toBe(3);
    var subs = [[2, 4, 1n], [4, 4, 2n], [1, 4, 3n]];
    for (let i = 0; i < 3; i++) {
        expect(packet.sub[i].version).toBe(subs[i][0]);
        expect(packet.sub[i].type   ).toBe(subs[i][1]);
        expect(packet.sub[i].value  ).toBe(subs[i][2]);
    }
});

PACKET_VERSIONS = [
    ['8A004A801A8002F478', 16],
    ['620080001611562C8802118E34', 12],
    ['C0015000016115A2E0802F182340', 23],
    ['A0016C880162017C3686B18A3D4780', 31],
];

test('test versions', () => {
    var bits = new tools.BITS();
    for (c of PACKET_VERSIONS) {
        var packet = bits.parse(c[0]);
        expect(bits.versionSum(packet)).toBe(c[1]);
    }
});

PACKET_EVALUATIONS = [
    ['C200B40A82', 3n],
    ['04005AC33890', 54n],
    ['880086C3E88112', 7n],
    ['CE00C43D881120', 9n],
    ['D8005AC2A8F0', 1n],
    ['F600BC2D8F', 0n],
    ['9C005AC2F8F0', 0n],
    ['9C0141080250320F1802104A08', 1n],
    ['800141080250320F1802104A08', 8n],
];
test('test eval', () => {
    var bits = new tools.BITS();
    for (c of PACKET_EVALUATIONS) {
        var packet = bits.parse(c[0]);
        expect(bits.eval(packet)).toBe(c[1]);
    }
});

