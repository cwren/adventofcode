#!javascript

var tools = {
    parseLine: function(s) {
        return s.split("").map(n => parseInt(n));
    },

    find_lows: function (h) {
        return  tools.find_low_points(h).map(p => h[p[0]][p[1]]);
    },

    find_low_points: function (h) {
        var lows = [];
        var i_max = h.length;
        var j_max = h[0].length;
        for (var i = 0; i < i_max; i++) {
            for (var j = 0; j < j_max; j++) {
                let v = h[i][j];
                let n = [];
                if (i > 0)           { n.push(h[i - 1][j]); }
                if (i < (i_max - 1)) { n.push(h[i + 1][j]); }
                if (j > 0)           { n.push(h[i][j - 1]); }
                if (j < (j_max - 1)) { n.push(h[i][j + 1]); }
                let min = n.reduce((m, s) => s < m ? s : m, v + 1)
                if (v < min) { lows.push([i, j]); }
            }
        }
        return lows;
    },

    compute_threat: function (lows) {
        return lows.map(n => n + 1).reduce((a, v) => a + v, 0);
    },

    search: function (p, h, visited) {
        var i = p[0];
        var j = p[1];
        if (i < 0
            || i > (h.length-1)
            || j < 0
            || j > (h[i].length-1)
            || h[i][j] == 9
            || visited[i][j]
           )
        {
            return 0;
        }

        visited[i][j] = true;
        return (1
                + tools.search([i - 1, j], h, visited)
                + tools.search([i + 1, j], h, visited)
                + tools.search([i, j - 1], h, visited)
                + tools.search([i, j + 1], h, visited)
               );
    },
    
    find_basins: function (h) {
        var visited = [];
        for (var i = 0; i < h.length; i++) {
            visited.push(new Array(h[0].length).fill(false));
        }
        var basins = [];
        var lp = tools.find_low_points(h);
        for (i in lp) {
            basins.push(tools.search(lp[i], h, visited));
        }
        return basins;
    },

    compute_score: function (a) {
        return a.sort((a,b) => b - a).slice(0, 3).reduce((s, v) => s * v, 1);
    },
    
};    
module.exports = tools;

function main() {
    var lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('009.txt')
    });
    var height_map = [];
    lineReader.on('line', (line) => {
        height_map.push(tools.parseLine(line));
    });

    lineReader.on('close', () => {
        var lows = tools.find_lows(height_map);
        var threat = tools.compute_threat(lows);
        console.log(`a: ${threat} `);

        var basins = tools.find_basins(height_map);
        var score = tools.compute_score(basins);
        console.log(`b: ${score} `);
    });
};

if (require.main === module) {
    main();
}
