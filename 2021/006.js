#!javascript

var tools = {
    parseLine: function(s) {
        var school = new Array(9).fill(0);
        s.split(",")
            .map(n => parseInt(n))
            .forEach(f => school[f] += 1);
        return school;
    },

    tick: function(previous) {
        var school = [...previous];
        var spawn = school.shift();
        school.push(spawn);
        school[6] += spawn;
        return school;
    },

    census: function(school) {
        return school.reduce((a,v) => a + v, 0);
    },
};    
module.exports = tools;

function main(days) {
    var lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('006.txt')
    });
    var school;
    lineReader.on('line', (line) => {
        school = tools.parseLine(line);
    });

    lineReader.on('close', () => {
        for (var t = 0; t < days; t ++) {
            school = tools.tick(school);
        }
        console.log(`# of fish after ${days} days: ${tools.census(school)}`);
    });
};

if (require.main === module) {
    var arg = parseInt(process.argv[process.argv.length - 1])
    var days = Number.isNaN(arg) ? 80 : arg;
    main(days);
}
