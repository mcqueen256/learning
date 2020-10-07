let message
message = 'abc';

// these are both ways to do type assertion, you choose.
let endsWithC = (<string>message).endsWith('c');
let alternativeWay = (message as string).endsWith('c');

let log = function(message) {
    console.log(message);
}


//hell yeah! arrow functions!!
let doLog = (message) => {
    console.log(message);
}

let strange = msg => console.log(msg);
