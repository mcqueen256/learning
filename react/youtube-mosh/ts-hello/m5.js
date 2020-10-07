class Point {
    // typescript generates the fields
    constructor(_x, _y) {
        this._x = _x;
        this._y = _y;
    }
    //functions
    draw() {
        console.log('X: ' + this._x, ', Y: ' + this._y);
    }
    // getX() {
    //     return this.x;
    // }
    // setX(value) {
    //     if (value < 0) {
    //         throw new Error("value cannot be less than 0.");
    //     }
    //     this.x = value;
    // }
    get x() {
        return this._x;
    }
    set x(value) {
        if (value < 0) {
            throw new Error("value cannot be less than 0.");
        }
        this._x = value;
    }
}
let point = new Point(1, 2);
// point.x = 10; // not accessable any more
point.draw();
let x = point.x;
point.x = 10;
// properties
// camal notation to name fields
