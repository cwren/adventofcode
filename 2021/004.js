#!javascript

var tools = {
    read_balls: function(s) {
        return s.split(",").map(item => parseInt(item))
    },
    
    Board: class {
        constructor (size = 5) {
            this.size = size;
            this.data = new Array();
            this.done = false;
            this.clear_marks();
        }

        get num_plays () {
            return this.history.length;
        }
        
        clear_marks () {
            this.done = false;
            this.history = new Array();
            this.mark = new Array();
            for (var i = 0; i < this.size; i++) {
                this.mark.push(new Array(this.size).fill(false))
            }
        }
        
        to_array () {
            return this.data;
        }
        
        from_array (a) {
            this.data = a;
            this.clear_marks();
        }
        
        parse (s) {
            if (!s) {
                return false;
            }
            if (this.data.length >= this.size) {
                throw 'This board is full';
            }
            var row = s.split(" ").filter(s => s != "");
            if (row.length != 5) {
                throw `malformed board line: ${s}`;
            }
            row = row.map(item => parseInt(item))
            if (row.some(item => isNaN(item))) {
                throw `malformed board line: ${s}`;
            }
            this.data.push(row)
            return this.data.length === 5
        }
        
        is_win() {
            var m = this.mark;
            return [
                // check all the rows
                m.map(r => r.every(b => b)).some(b => b),
                // check all the columns
                m[0].map((e, i) => m.map(r => r[i]))
                    .map(col => col.every(b => b))
                    .some(b => b)
            ].some(b => b)
        }

        get score () {
            if (this.done) {
                var last_ball = this.history[this.history.length - 1]
                var sum = 0;
                this.mark.forEach((r, i) => {
                    r.forEach((b, j) => {
                        if (!b) {
                            sum += this.data[i][j]
                        }
                    })
                })
                return sum * last_ball;
            } else {
                return -1;
            }
        }
        
        play (b) {
            if (this.done) {
                return true;
            }
            this.history.push(b);
            for (var i = 0; i < this.size; i++) {
                if (this.data[i].includes(b)) {
                    var j = this.data[i].indexOf(b);
                    this.mark[i][j] = true;
                    if (this.is_win()) {
                        this.done = true;
                        return true;
                    }
                }
            }
            return false;
        }
    }
};    
module.exports = tools;

function main() {
    var lineReader = require('readline').createInterface({
        input: require('fs').createReadStream('004.txt')
    });
    
    var first = true;
    var board = null;
    var balls = null;
    var boards = [];
    lineReader.on('line', (line) => {
        if (first) {
            first = false;
            balls = tools.read_balls(line);
        } else {
            if (board == null) {
                // start a new board for the fresh input
                board = new tools.Board();
                boards.push(board);
            }
            if (board.parse(line)) {
                // this board is done
                board = null;
            }
        }
    });
    
    lineReader.on('close', () => {
        // play all the balls
        balls.forEach(ball => {
            boards.forEach(board => board.play(ball))
        })

        var num_plays = boards.map(b => b.num_plays);
        var first = Math.min(...num_plays);

        var firsts = boards.filter(b => b.num_plays == first);

        var first_scores = firsts.map(b => b.score);
        var winning_score = Math.max(...first_scores);

        var winner = firsts.filter(b => b.score == winning_score);

        console.log(`numvber of winners: ${winner.length}`);
        console.log(`first winner ${winner[0].score}`);

        var finishers = boards.filter(b => b.score > 0);
        var finished_plays = finishers.map(b => b.num_plays);
        var last = Math.max(...finished_plays);
        var lasts = boards.filter(b => b.num_plays == last);
        var last_scores = lasts.map(b => b.score);
        var losing_score = Math.min(...last_scores);
        var loser = lasts.filter(b => b.score == losing_score);

        console.log(`number of losers: ${loser.length}`);
        console.log(`first loser ${loser[0].score}`);
    });
};

if (require.main === module) {
  main();
}
