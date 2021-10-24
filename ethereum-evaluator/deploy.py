# Build and Deploy Contract
import os
import sys
import json

contract = sys.argv[1]
os.system(f"solc --overwrite --bin ethereum-evaluator/{contract}.sol -o ethereum-evaluator/build")
os.system(f"solc --overwrite --abi ethereum-evaluator/{contract}.sol -o ethereum-evaluator/build")

with open("ethereum-evaluator/auth.json","r") as f: auth = json.load(f)
with open(f"ethereum-evaluator/build/{contract}.bin","r") as f: b = f.read()
with open(f"ethereum-evaluator/build/{contract}.abi","r") as f: a = f.read()

jsscript = f"""
personal.unlockAccount("{auth["account"]}","{auth["password"]}")
eth.defaultAccount = "{auth["account"]}"

var contract = eth.contract({a})
var bytecode = "0x{b}"

var deploy = {{from:"{auth["account"]}", data:bytecode, gas: 2000000}}

var getPartialInstance = contract.new(deploy,function(e, c){{
 if(!e) {{
   if(c.address) {{
     console.log(c.address);
   }}     
 }} else {{
     console.log(e);     
    }}  
}})
"""

with open(f"ethereum-evaluator/build/{contract}.js","w") as f: f.write(jsscript)
os.system(f"geth --goerli js ethereum-evaluator/build/{contract}.js 2>>ethereum-evaluator/build/debug.log >ethereum-evaluator/build/{contract}.txt")

with open(f"ethereum-evaluator/build/{contract}.txt","r") as f: assert f.read().split("\n")[0].strip()[:2] == "0x"
