const express = require('express')
const app = express()
const port = 8000;
var cors = require('cors');
app.use(cors());

app.post('/records', (req, res) => {
  res.send('Hello World!')
})

app.get('/request', (req, res) => {
  console.log(req.params);
  res.redirect('http://localhost:3000/newprocess');
})

/*
app.get('/records/:id',(req,res) => {
    console.log(req.params.id);

    var times = [70000,2373,5.9,65,4370];
    var data = [["time","Ethereum","Salana","Local","Cloudfare","Polka"]];
    for (var i = 0; i < req.params.id; i++) {
      var mili = i*1000;
      var d = [i];
      for (var j = 0; j < times.length; j++) {
        d.push(Math.floor(mili/times[j]))
      }
      data.push(d);
    }

    res.json({data: data});
})*/


app.get('/records/:id',(req,res) => {
  console.log(req.params.id);
  var jas = req.query.js.split(",");
  var times = [2373,5.9,65,4370,70000];
  var linear = [false,true,true,false,false];
  var data = [[["time","Solana"],[0,0]],[["time","Local"],[0,0]],[["time","Cloudflare"],[0,0]],[["time","Polkadot"],[0,0]],[["time","Ethereum"],[0,0]]];
  var mili = req.params.id*1000;
  for (var i = 0; i < times.length; i++) {
    var num = Math.floor(mili/times[i]);
    if (num > 0) {
      data[i].pop();
    }
    if (num > 120) num = 120;
    for (var j = jas[i]; j < num; j++) {
      var exectime;
      if (j > 3 || linear[i]) exectime = ((Math.random()-0.5)*times[i])/100 + times[i];
      else exectime = times[i]*(5.0-j)/3;
      //console.log(exectime, times[i], j);}
      data[i].push([j,exectime]);
    }
  }

  res.json({data: data});
})


app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`)
})
