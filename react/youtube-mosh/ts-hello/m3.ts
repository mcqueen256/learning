// for functions, if we have a tone of params maybe turn them into an object>

// inline annotation
// let drawPoint = (point: { x: number, y: number}) => {

// }

// drawPoint({
//     x: 1,
//     y: 2
// }) // okay ... 

// drawPoint({
//     name: "jim"
// }) // ahh! this is bad!


// Lets use interfaces!!
// interface Point_ols {
//     x: number,
//     y: number
// }

// let drawPoint_old = (point: Point) => {
//     // ..
// }

// drawPoint({
//     x: 1,
//     y: 2
// }) // cool!