# Run Existing Contract
import os
import sys
import json

contract = sys.argv[1]
function = sys.argv[2]

code = sys.stdin.buffer.read().hex()
codestr = "[" + ",".join([f'"0x{code[i*2:i*2+2]}"' for i in range(len(code)//2)]) + "]"

with open("ethereum-evaluator/auth.json","r") as f: auth = json.load(f)
with open(f"ethereum-evaluator/build/{contract}.abi","r") as f: a = f.read()
with open(f"ethereum-evaluator/build/{contract}.txt","r") as f: addr = f.read().split("\n")[0].strip()

jsscript = f"""
personal.unlockAccount("{auth["account"]}","{auth["password"]}")
eth.defaultAccount = "{auth["account"]}"

var contract = eth.contract({a})
var instance = contract.at("{addr}")

instance.log(function(error, result){{
    if (!error) {{
        console.log(result.args.time, result.args.b);
    }}
}});
var x = instance.{function}({codestr if code else ""});
function wait()  {{
    setTimeout(wait,100000)
}}
wait();

"""
with open("tmp.js","w") as f: f.write(jsscript)
os.system(f"geth --goerli js tmp.js 2>>ethereum-evaluator/build/debug.log")
os.system("rm tmp.js")
