#lang racket

(define (parse line)
  (let ((matches (regexp-match* #rx"([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)" (string-trim line) #:match-select cdr)))
    (map string->number (car matches))))

(define (my-range start end)
  (let ((step (sgn (- end start))))
    (inclusive-range start end step)))

(define (draw line)
  (match-let (((list x1 y1 x2 y2) line))
    (let* ((size (+ 1 (max (abs (- x2 x1)) (abs (- y2 y1)))))
           (range1 (if (= x1 x2) (make-list size x1) (my-range x1 x2)))
           (range2 (if (= y1 y2) (make-list size y1) (my-range y1 y2))))
      (map list range1 range2))))

(define (freq lst)
  (let ((ht (make-hash)))
    (for ((v lst))
      (hash-update! ht v add1 0))
    ht))

(define (find-duplicates)
  (let ((lines (apply append (map draw (map parse (file->lines "input.txt"))))))
    (count (lambda (v) (> v 1)) (hash-values (freq lines)))))