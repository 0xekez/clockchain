# Run Existing Contract
import os
import sys
import json

contract = sys.argv[1]

with open("auth.json","r") as f: auth = json.load(f)
with open(f"build/{contract}.abi","r") as f: a = f.read()
with open(f"build/{contract}.txt","r") as f: addr = f.read().split("\n")[0].strip()

jsscript = f"""
personal.unlockAccount("{auth["account"]}","{auth["password"]}")
eth.defaultAccount = "{auth["account"]}"

var contract = eth.contract({a})
var instance = contract.at("{addr}")
instance.methods.getResult().call(function(err,res){{
    console.log(res);
}});
"""
with open("tmp.js","w") as f: f.write(jsscript)
os.system(f"geth --goerli js build/{contract}.js 2>>build/debug.log")
#os.system("rm tmp.js")
