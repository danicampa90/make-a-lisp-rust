
(def! #t true)
(def! #f false)

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