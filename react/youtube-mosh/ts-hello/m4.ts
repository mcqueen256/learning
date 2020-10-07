class Point {
    // fields
    private x: number;
    private y: number;

    // special functions
    constructor(x?: number, y?: number) {
        this.x = x;
        this.y = y;
    }

    //functions
    draw() {
        console.log('X: ' + this.x, ', Y: ' + this.y);
    }
    getDistance(another: Point) {
        // ..
    }
}

let point = new Point(7, 2);
// point.x = 10; // not accessable any more
point.draw();

// access modifiers {public, private, protected}

