var Point = /** @class */ (function () {
    // special functions
    function Point(x, y) {
        this.x = x;
        this.y = y;
    }
    //functions
    Point.prototype.draw = function () {
        console.log('X: ' + this.x, ', Y: ' + this.y);
    };
    Point.prototype.getDistance = function (another) {
        // ..
    };
    return Point;
}());
var point = new Point(7, 2);
// point.x = 10; // not accessable any more
point.draw();
// access modifiers {public, private, protected}
