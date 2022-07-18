let config = require("../config.json");
let main = require("../ctt.js");
module.exports = (input) => {
    input = input.substring(1);

    input = input.split(" ");


    switch (input[0]){
        case "connect":
            connect(input.slice(1))
            break
        case "disconnect":
            break
        default:
            console.log("Unknown command \"" + input[0] + "\"")
    }
}


function connect(input){
    switch (input[0]){
        case "preset":
            main.setSettings(config.preset[input[1]]);
            break;
        default:
            if (!input[1]){
                console.log("ERROR: please supply an ip/domain to connect to!");
                return;
            } else {
                main.setSettings(config.defaults)
                let settings = main.getSettings();
                settings.host = input[1];
                main.setSettings(settings);
            }
            break
    }
}