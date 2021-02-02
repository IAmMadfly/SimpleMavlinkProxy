const dgram = require("dgram");

const socket = dgram.createSocket("udp4");
var counter = 0;

setInterval(
    function() {
        socket.send(Buffer.from("Hello world:"+counter), 14552);
        counter += 1;
    },
    1000
);

setInterval(
    function() {
        socket.send(Buffer.from("Hello Karl:"+counter), 14553);
        counter += 1;
    },
    700
);