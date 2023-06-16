; we define most of the operands in terms of a minimum of native functions.
; efficiency here is NOT the objective, and it's much easier to maintain these
; simple definitions in lisp than full rust implementations of all of these
; functions (with all the type conversions and AST matching).

(def! #t true)
(def! #f false)

; math ops
; a - (-b) = a + b
(def! + (fn* (a b) 
  (- a (- 0 b)) 
))

; Bool ops
(def! empty? (fn* (l) 
  (= 0 (count l)) 
))
(def! not (fn* (a) 
  (if a #f #t)
))
(def! or (fn* (a b) 
  (nand(not a) (not b))
))
(def! and (fn* (a b) 
  (not (nand a b))
))

; comparison ops, based on < and =
(def! <= (fn* (a b)
  (or (< a b) (= a b))
))
(def! >= (fn* (a b)
  (not (< a b) )
))
(def! <> (fn* (a b)
  (not (= a b) )
))
(def! > (fn* (a b)
  (and (>= a b) (<> a b) )
))