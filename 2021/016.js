#!javascript

var tools = {
    BITS: class {

        stringToBits(s) {
            var hex = s.split('').map(x => parseInt(x, 16));
            var bits = [];
            hex.forEach(x => bits.push((x >> 3) & 0x1, (x >> 2) & 0x1, (x >> 1) & 0x1, x & 0x1));
            return bits;
        }
        
        readNumber (n, bits) {
            let value = 0;
            for (let i = 0; i < n; i++) {
                value <<= 1;
                value |= bits.shift();
            }
            return value;
        }
        
        readBigNum (bits) {
            let value = 0n;
            let done = false;
            let bytes = 0;
            while (!done) {
                bytes += 1;
                done = bits.shift() == 0;
                value <<= 4n;
                value += BigInt(this.readNumber(4, bits));
            }
            return value;
        }
        
        readLengthSubs(bits) {
            let subs = [];
            let n = this.readNumber(15, bits);
            let start = bits.length;
            while (bits.length > (start - n)) {
                subs.push(this.readPacket(bits));
            }
            return subs;
        }

        readCountSubs(bits) {
            let subs = [];
            let n = this.readNumber(11, bits);;
            for (let i = 0; i < n; i++) {
                subs.push(this.readPacket(bits));
            }
            return subs;
        }

        readSubs(bits) {
            if (bits.shift() == 0) {
                return this.readLengthSubs(bits);
            } else {
                return this.readCountSubs(bits);
            }
        }

        eatPadding (bits) {
            while ((bits.length % 4) != 0) {
                bits.shift();
            }
        }

        readPacket (bits) {
            var packet = {};
            packet.version = bits.shift() << 2 | bits.shift() << 1 | bits.shift();
            packet.type = bits.shift() << 2 | bits.shift() << 1 | bits.shift();
            switch (packet.type) {
            case 4:
                packet.value = this.readBigNum(bits);
                break
            default:
                packet.sub = this.readSubs(bits);
                break;
            }
            // this.eatPadding(bits); // desc says yes, but examples say no
            return packet;
        }

        parse (s) {
            return this.readPacket(this.stringToBits(s));
        }

        versionSum (p) {
            var total = p.version;
            if ('sub' in p) {
                for (let s of p.sub) {
                    total += this.versionSum(s);
                }
            }
            return total;
        }

        eval (p) {
            if (p.type == 4) {
                return p.value;
            }
            let v = p.sub.map(s => this.eval(s));
            switch (p.type) {
            case 0:  // sum
                return v.reduce((v, a) => a + v, 0n);
            case 1:  // product
                return v.reduce((v, a) => a * v, 1n);
            case 2:  // min
                return v.reduce((v, a) => v < a ? v : a, Infinity);
            case 3:  // max
                return v.reduce((v, a) => v > a ? v : a, -Infinity);
            case 5:  // >
                return v[0] > v[1] ? 1n : 0n;
            case 6:  // <
                return v[0] < v[1] ? 1n : 0n;
            case 7:  // ==
                return v[0] == v[1] ? 1n : 0n;
            default:
                console.log(`unknown packet type {p.type}`)
            }
        }
    },
};    
module.exports = tools;

function main() {
    let lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('016.txt')
    });
    
    var bits = new tools.BITS();
    var packet = {};
    lineReader.on('line', (line) => {
        packet = bits.parse(line);
    });
    
    lineReader.on('close', () => {
        console.log(`check: ${bits.versionSum(packet)}`);
        console.log(` eval: ${bits.eval(packet)}`);
    });
};

if (require.main === module) {
  main();
}
