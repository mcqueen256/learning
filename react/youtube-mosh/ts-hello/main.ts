function log(message) {
  console.log(message);
}

var message = 'hello world';

log(message);

var number = 1;
let count = 2;

function doSomething() {
  for (let i = 0; i < 5; i++) {
    console.log(i);
  }
  let i = 6;
  console.log('Finally: ' + i);
}

doSomething();
