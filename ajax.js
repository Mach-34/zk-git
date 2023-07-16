const https = require("https");

const $ = module.exports = {
    post: post,
    get: get,
    getURL: getURL
};

function raw2json(raw) {
    try {
        return JSON.parse(raw);
    } catch (err) {
        console.log("ERREUR L21 : Parse data impossible");
        console.log(err.stack);
        console.log("\033[91mCa commence bien le Launcher plante...\033[00m");
        return process.exit();
    }
}

function getURL(url, success) {
    $.get({
        url: url,
        success: function(res, data) {
            const json = raw2json(data);

            if (success)
                success(json)
        }
    })
}

function get(option) {
    option.method = "GET";
    return ajax(option);
}

function post(option) {
    option.method = "POST";
    return ajax(option);
}

function ajax(option) {
    const data = (option.data) ? querystring.stringify(option.data) : "",
        context = (option.context) ? option.context : {};

    const options = {
        host: "api.github.com",
        port: "443",
        path: option.url,
        method: (option.method == "GET") ? "GET" : "POST",
        headers: {
            "User-Agent": "NodeJS for git api",
            "Content-Type": "application/x-www-form-urlencoded",
            "Accept": "application/json",
            "Content-Length": data.length
        }
    };

    const req = https.request(options, function(res) {
        res.setEncoding("utf8");
        let content = "";

        res.on("data", function(chunk) {
            content += chunk;
        });

        res.on("end", function() {
            if (option.success) {
                option.success(res, content, context);
            }
        });
    });

    req.on("error", function(e) {
        console.log("\033[91merror: " + e.message + "\033[00m");
        setTimeout(function() {
            return ajax(option);
        }, 1500);
    });

    req.write(data);
    req.end();
}