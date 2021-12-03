#!javascript

var lineReader = require('readline').createInterface({
  input: require('fs').createReadStream('002.txt')
});

var x = 0
var y = 0

lineReader.on('line', (line) => {
    [o, n] = line.split(" ");
    n = parseInt(n)
    switch (o) {
    case 'forward':
        x += n;
        break;
    case 'up':
        y -= n;
        break;
    case 'down':
        y += n;
        break;
    }
});
lineReader.on('close', () => {
    console.log(`${x} * ${y} = ${x*y}`);
});
