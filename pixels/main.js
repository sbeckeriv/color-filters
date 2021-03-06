var child_process = require('child_process');

exports.handler = function(event, context) {

  var env ={};
  env.LD_LIBRARY_PATH=process.env["LD_LIBRARY_PATH"];

  env.PATH = process.env['PATH'] + ':' + process.env['LAMBDA_TASK_ROOT']
  env.AWS_ACCESS_KEY_ID="";
  env.AWS_SECRET_ACCESS_KEY="";
  env.RUST_BACKTRACE=1;
  child_process.exec(
    './image "'+event["imageUrl"]+'"',
    {env:env},
    (error, stdout, stderr) => {
      if (error) {
        context.fail(JSON.stringify({"message": error.message}));
      }else{
        context.succeed({ "images": stdout.replace(/"/g,"").split("\n")});
      }
      return;
    });
}


