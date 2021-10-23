## Example of fibonacci compilation

The Lisp code pre compilation:

```lisp
(let fib (fn (n)
	     (if (lt n 2)
		 n
		 (add (fib (sub n 1)) (fib (sub n 2))))))
(fib 4)
```

The Lisp code after being compiled:

```
begin:
  pi 4
  jump fib

fib:         | n
  copy       | n n
  pi 2       | n n 2
  jlt done   | n
  copy       | n n
  pi 1       | n n 1
  sub        | n n-1
  call fib   | n fib(n-1)
  rot 1      | fib(n-1) n
  pi 2       | fib(n-1) n 2
  sub        | fib(n-1) n-2
  call fib   | fib(n-1) fib(n-2)
  add        | fib(n)
done:
  ret        | fib(n)
```

The bytecode after having its labels removed and jumps filled with
their relative versions:

```
pi 4
jump 1
copy
pi 2
jlt 10
copy
pi 1
sub
call -6
rot 1
pi 2
sub
call -10
add
ret
```

## Example of is-prime compilation

The Lisp code before being compiled:

```lisp
(let is-prime-helper (fn (n i)
		  (if (gt (* i i) n)
		      1
		      (if (eq (mod n i) 0)
			  0
			  (is-prime-helper n (+ i 2))))))

(let is-prime (fn (n)
		  (if (lt n 3)
		      1
		      (if (eq (mod n 2) 0)
			  0
			  (is-prime-helper n 3)))))

(is-prime 11)
```

The bytecode the Lisp gets compiled to:

```
is-prime-helper:          | n i
	copy                  | n i i
	copy                  | n i i i
	mul                   | n i i*i
	rot 2                 | i n i*i
	rot 1                 | i i*i n
	copy                  | i i*i n n
	rot 2                 | i n i*i n
	jgt iph-true          | i n
	copy                  | i n n
	rot 2                 | n i n
	rot 1                 | n n i
	copy                  | n n i i
	rot 2                 | n i n i
	mod                   | n i n%i
	pi 0                  | n i n%i 0
	jeq iph-false         | n i
	pi 2                  | n i+2
	call is-prime-helper  | is-prime-helper(n, i+2)
	ret
iph-true:                 | n i
	pop                   | n
	pop                   |
	pi 1                  | 1
	ret
iph-false:                | n i
	pop                   | n
	pop                   |
	pi 0                  | 0
	ret

is-prime:                 | n
    copy                  | n n
	pi 3                  | n n 3
	jlt ip-true           | n
	copy                  | n n
	pi 2                  | n n 2
	mod                   | n n%2
	pi 0                  | n n%2 0
	jeq ip-false          | n
	pi 3                  | n 3
	call is-prime-helper  | is-prime-helper(n, 3)
	ret
ip-true:
	pop                   |
	pi 1                  | 1
	ret
ip-false:
	pop                   |
	pi 0                  | 0
	ret
```
