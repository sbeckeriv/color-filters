var child_process = require('child_process');

exports.handler = function(event, context) {
  console.log(event["imageUrl"]);
  var env = Object.create( process.env );
  env.AWS_ACCESS_KEY_ID="";
  env.AWS_SECRET_ACCESS_KEY="";

  child_process.exec('./image "'+event["imageUrl"]+'"', {env:env}, (error, stdout, stderr) => {
    if (error) {
      console.error(`exec error: ${error}`);
      return;
    }
    console.log(`stdout: ${stdout}`);
    console.log(`stderr: ${stderr}`);
  });
}


