#!/usr/bin/env node

let readlineSync = require("readline-sync");
let commandManager = require("./src/commandManager.js");
let messageManager = require("./src/messageManager.js")
let config = require("./config.json");



let input;

let user = {
    username: null,
    server: null
}

let settings = {
    host: null,
    port: null,
    routes: {
        login: null,
        register: null,
        user: null,
        sendMessage: null
    }
}
/**
 *          "host": "127.0.0.1",
            "port": 3000,
            "routes": {
                "login": "/auth/login",
                "register": "/auth/register",
                "user": "/user",
                "send_message": "/channels/{}"
            }
 */
function setSettings(cfg){
    settings = cfg
}

function getSettings(){
    return settings;
}


module.exports = {
 setSettings
}


while (true) {
    input = readlineSync.question(`${user.username}@${user.server}>`);
    if (input == "") continue;

    if (input[0] == "/"){
        commandManager(input);
    } else {
        sendMessage(input)
    }

}