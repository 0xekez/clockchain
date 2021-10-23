# Build and Deploy Contract
import os
import sys
import json

contract = sys.argv[1]
os.system(f"solc --overwrite --bin {contract}.sol -o build")
os.system(f"solc --overwrite --abi {contract}.sol -o build")

with open("auth.json","r") as f: auth = json.load(f)
with open(f"build/{contract}.bin","r") as f: b = f.read()
with open(f"build/{contract}.abi","r") as f: a = f.read()

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

with open(f"build/{contract}.js","w") as f: f.write(jsscript)
os.system(f"geth --goerli js build/{contract}.js 2>>build/debug.log >build/{contract}.txt")

with open(f"build/{contract}.txt","r") as f: assert f.read().split("\n")[0].strip()[:2] == "0x"
