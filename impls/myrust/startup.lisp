; we define most of the operands in terms of a minimum of native functions.
; efficiency here is NOT the objective, and it's much easier to maintain these
; simple definitions in lisp than full rust implementations of all of these
; functions (with all the type conversions and AST matching).

(def! #t true)
(def! #f false)

; Bool ops
(def! empty? (fn* (l) 
  (= 0 (count l)) 
))

; from step 6
(def! load-file (fn* (f) (eval (read-string (str "(do " (slurp f) "\nnil)")))))

; debugging
(def! set-trace (fn* (enabled?) (do (set-trace-calls enabled?) (set-trace-native-calls enabled?))))


; step 7 list functions
(def! first (fn* (list) 
  (if
    (< 0 (count list))
    (nth list 0)
    nil
  )
))

(defmacro! cond (fn* (& xs) (if (> (count xs) 0) (list 'if (first xs) (if (> (count xs) 1) (nth xs 1) (throw "odd number of forms to cond")) (cons 'cond (rest (rest xs)))))))


; some step9 functions can be defined in here

(def! empty? (fn* (list) 
  (= (count list) 0)
))

(def! last (fn* (list) 
  (if (empty? list)
    nil
    (nth list (- (count list) 1))
  )
))

; MY TESTS
(defmacro! test (fn* (name expr1 expr2)
  `(if (= ~expr1 ~expr2) nil (throw (str "TEST FAILED: " ~name "\nfirst:" ~expr1 "\nsecond:" ~expr2)))
))


(test 'hi 1 1)

(test "empty? - test 1" (empty? '(1 2 3)) #f)
(test "empty? - test 2" (empty? '()) #t)
(test "last - test 1" (last '(1 2 3)) 3)
(test "last - test 2" (last '(())) '())
(test "last - test 1" (last '()) nil)

(test "booleans - test 1", (and #t #t) #t)
(test "booleans - test 1", (and #f #t) #f)



(def! sequential? (fn* (args) (or (list? args)(vector? args))))

; a tail-call-optimizable function that takes a list in the original MAL apply specs, and expands it into a simple (flat) list of arguments.
; basically it concatenates the last element of args (which must be a list) to the previous elements.
; it's a bit overcomplicated here, should be rewritten.
; initial call should have 'existing = '()

(def! quote-list-items (fn* (list)
  (if (empty? list) 
    '()
    (cons `(quote ~(first list)) (quote-list-items (rest list)))
  )
))
(def! concat-apply-args (fn* (existing args) 
  (cond 
    (and (= (count args) 1) (sequential? (first args))) (concat existing (quote-list-items (first args)))
    (= (count args) 1) (throw "last argument in apply is not a list")
    "else" (concat-apply-args (concat existing (list `(quote ~(first args)))) (rest args) )
  )
))

(def! concat-apply-macro-args (fn* (existing args) 
  (cond 
    (and (= (count args) 1) (sequential? (first args))) (concat existing (first args))
    (= (count args) 1) (throw "last argument in apply is not a list")
    "else" (concat-apply-args (concat existing (list (first args))) (rest args) )
  )
))

(test "concat-apply-args - test 1" (concat-apply-args '() '(1 a (3 4))) '('1 'a '3 '4) )
(test "concat-apply-args - test 2" (concat-apply-args '() '(())) '() )
(test "concat-apply-args - test 3" (concat-apply-args '() '(1 2 ())) '('1 '2) )
(test "concat-apply-args - test 4" (concat-apply-args '() '((1))) '('1) )

; apply, implemented only in terms of existing functions :)
(def! apply (fn* (fn & args) 
  (eval (cons fn (if (macro? fn) (concat-apply-macro-args '() args) (concat-apply-args '() args))))
))

(test "apply - test 1" (apply '+ 1 2 '()) 3 )
(test "apply - test 2" (apply 'list 1 2 '(3 4) ) '(1 2 3 4) )

; map defined in terms of apply
(def! map (fn* (fn lst) (
  if (empty? lst)
    '()
    (cons (apply fn (first lst) '()) (map fn (rest lst)))
  
)))


(def! true? (fn* (a) (= a true)))
(def! false? (fn* (a) (= a false)))
(def! nil? (fn* (a) (= a nil)))

(defmacro! vector (fn* (& args) `(vec '(~@args)) ))

(defmacro! hash-map (fn* (& args) `(assoc {} ~@args ) ))

(def! time-ms (fn* (coll &elements) (throw "not implemented"))) ; get system time in ms
(def! meta (fn* (node) (throw "not implemented"))) ; returns metadata
(def! with-meta (fn* (node metadata) (throw "not implemented"))) ; sets metadata
(def! seq (fn* (node metadata) (throw "not implemented"))) ; sets metadata
(def! conj (fn* (node metadata) (throw "not implemented"))) ; sets metadata

; string?, number?, seq, and conj

(def! *ARGV* (if (> (count (get-argv)) 1)
  (rest (rest (get-argv)))
  (rest (get-argv)
)))

;(trace #t)