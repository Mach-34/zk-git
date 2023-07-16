const crypto = require("crypto"),
      fs = require("fs"),
      path = require("path"),
      getURL = require("./ajax.js").getURL;

const apiJSON = [];
//https://api.github.com/
const hashs = [
  "8d66139b3acf78fa50e16383693a161c33b5e048",
  "4ef57de8e81c8415d6da2b267872e602b1f28cfe",
  "13b54c0bab5e7f7a05398d6d92e65eee2b227136",
  "218a8f506fcd3076fad059ec42d4656c635a8171"
];

let loaded = 0;

const USEAPI = true; /*  becarful low limit on repl.it */

for (let i = 0; i < hashs.length; i++) {
  if (!USEAPI) {
    apiJSON[i] = JSON.parse(fs.readFileSync(`a${i+1}.json`));
    console.log(`A${i+1}:`);
    getTreeSHA(apiJSON[i], false);

    if (i+1 === hashs.length) {
      console.log("\n\nPerl ouput:");
      for (let j = 0; j < hashs.length; j++)
        getTreeSHA(apiJSON[j], true);
    }
  } else {
    getURL("/repos/zestedesavoir/zds-site/git/trees/" + hashs[i], function(json) {
      loaded++; apiJSON[i] = json;
      if (loaded === hashs.length) {
        for (let i = 0; i < hashs.length; i++) {
          console.log(`A${i+1}:`); getTreeSHA(apiJSON[i], false);
        }
        console.log("\n\nPerl ouput:");
        for (let i = 0; i < 1; i++)
          getTreeSHA(apiJSON[i], true);
      }
    });
  }
}

function getTreeSHA(json, getPattern) {
  /*json.tree.sort((a, b) => { ---> not good see A3 & A4
    if (a.type !== b.type)
      if (a.type === "tree")
        return 1;
      else if (b.type === "tree")
        return -1;
    return a.path.charCodeAt(0) - b.path.charCodeAt(0)
  });*/

  let text = "";

  Object.values(json.tree).forEach(function (blob) {
      const sha = Buffer.from(blob.sha, "hex").toString(!getPattern ? "binary" : "hex");
      text += (+blob.mode) + " " + blob.path;
      //       ^ https://stackoverflow.com/a/54137728
      text += (!getPattern) ? ("\0" + sha) : (" " + sha + "\n");
  });
  console.log("Text: ", text);
  console.log("done")

  if (getPattern) return console.log(text.replace(/\0/g, ""));

  console.log("Original " + json.sha);
  const pattern = "tree " + text.length + "\0" + text;
  console.log("Actual : " + sha1(pattern));

  function sha1(data) {
      return crypto.createHash("sha1").update(data, "binary").digest("hex");
  }
}
