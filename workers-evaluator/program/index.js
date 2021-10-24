// curl -v -H "Content-Type: application/octet-stream" --data-binary @test.bc http://127.0.0.1:8787

addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
    if (request.method == "POST"){
	const buffer = await request.arrayBuffer()
	const bytecode = new Uint8Array(buffer)

	let pc = 0
	let call_stack = []
	let stack = []

	while (true) {
	    const inst_start = (pc * 5)
	    const opcode = bytecode[inst_start]
	    const immediate = bytecode[inst_start + 1] << 0 |
		  bytecode[inst_start + 2] << 8 |
		  bytecode[inst_start + 3] << 16 |
		  bytecode[inst_start + 4] << 24

	    // console.log(`${opcode} ${immediate}\t[${stack}]`)

	    switch (opcode) {
	    case 1:
		stack.push(immediate)
		break
	    case 2:{
		const last = stack[stack.length - 1]
		stack.push(last)
		break
	    }
	    case 3: {
		const right = stack.pop()
		const left = stack.pop()
		stack.push(left + right)
		break
	    }
	    case 4: {
		const right = stack.pop()
		const left = stack.pop()
		stack.push(left - right)
		break
	    }
	    case 5: {
		const right = stack.pop()
		const left = stack.pop()
		stack.push(left * right)
		break
	    }
	    case 6: {
		const right = stack.pop()
		const left = stack.pop()
		stack.push(left / right)
		break
	    }
	    case 7: {
		const right = stack.pop()
		const left = stack.pop()
		stack.push(left % right)
		break
	    }
	    case 8: {
		pc = pc + immediate - 1
		break
	    }
	    case 9: {
		const right = stack.pop()
		const left = stack.pop()
		if (left == right) {
		    pc = pc + immediate - 1
		}
		break
	    }
	    case 10: {
		const right = stack.pop()
		const left = stack.pop()
		if (left != right) {
		    pc = pc + immediate - 1
		}
		break
	    }
	    case 11: {
		const right = stack.pop()
		const left = stack.pop()
		if (left < right) {
		    pc = pc + immediate - 1
		}
		break
	    }
	    case 12: {
		const right = stack.pop()
		const left = stack.pop()
		if (left > right) {
		    pc = pc + immediate - 1
		}
		break
	    }
	    case 13: {
		const stackloc = stack.length - immediate - 1
		const tmp = stack[stackloc]
		stack[stackloc] = stack[stackloc + 1]
		stack[stackloc + 1] = tmp
		break
	    }
	    case 14: {
		call_stack.push(pc)
		pc = pc + immediate - 1
		break
	    }
	    case 15: {
		pc = call_stack.pop()
		break
	    }
	    case 16: {
		stack.pop()
		break
	    }
	    case 17: {
		const res = stack.pop()
		return new Response(`${res}\n`, {
		    headers: { 'content-type': 'text/plain' },
		})
	    }
	    default: {
		return new Response(`unrecognized opcode (${opcode})`, {
		    status: 400,
		    headers: { 'content-type': 'text/plain' },
		})
		break
	    }
	    }
	    pc += 1
	}
    }
}
