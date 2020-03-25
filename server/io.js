const io = require('socket.io')
let ioInstance = null;
module.exports = function getSocketIOInstance(server) {
    if(server) {
        ioInstance = io(server);
    }
    return ioInstance;
}