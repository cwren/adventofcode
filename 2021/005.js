#!javascript

var tools = {
    parseLine: function(s) {
        return s.split(" -> ")
            .map(c => c.split(","))
            .map(p => p.map(n => parseInt(n)))
    },

    isHorizontal: function(l) {
        return l[0][1] == l[1][1]
    },

    isVertical: function(l) {
        return l[0][0] == l[1][0]
    },

    isDiagonal: function(l) {
        return (Math.abs(l[0][0] - l[1][0]) == Math.abs(l[0][1] - l[1][1]));
    },

    CoordMap: class extends Map {
        get(key) {
            return super.get(JSON.stringify(key));
        }
        
        inc(key) {
            var k = JSON.stringify(key);
            if (!super.has(k)) {
                super.set(k, 1);
            } else {
                super.set(k, super.get(k) + 1);
            }
        }

        keys() {
            return Array.from(super.keys()).map(k => JSON.parse(k));
        }
    },

    SeaFloorMap: class {
        constructor () {
            this.history = [];
            this.coords = new tools.CoordMap();
        }

        get num_intersections () {
            return this.coords.keys()
                .filter(k => this.coords.get(k) > 1)
                .length;
        }
        
        draw (l, only_hv=true) {
            if (only_hv &&
                !tools.isHorizontal(l) &&
                !tools.isVertical(l)) {
                return;
            }
            if (!tools.isDiagonal(l) && 
                !tools.isHorizontal(l) && 
                !tools.isVertical(l)) {
                return;
            }
            var s = l[0];
            var e = l[1];
            var d = [0, 0];
            d = [0, 0];
            if (tools.isHorizontal(l) || tools.isDiagonal(l)) {
                d[0] =  s[0] < e[0] ? 1 : -1;
            }
            if (tools.isVertical(l) || tools.isDiagonal(l)) {
                d[1] =  s[1] < e[1] ? 1 : -1;
            }
            while(s[0] != e[0] || s[1] != e[1]) {
                this.coords.inc(s);
                s[0] += d[0];
                s[1] += d[1];
            }
            this.coords.inc(s);
        }
    }
};    
module.exports = tools;

function main() {
    var lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('005.txt')
    });
    var sfm = new tools.SeaFloorMap();
    lineReader.on('line', (line) => {
        sfm.draw(tools.parseLine(line), hv_only=false);
    });
    
    lineReader.on('close', () => {
        console.log(`num_interections: ${sfm.num_intersections}`);
    });
};

if (require.main === module) {
  main();
}
