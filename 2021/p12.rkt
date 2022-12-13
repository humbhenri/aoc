#lang racket
(define graph '((start . (A b))
                (A . (c b end))
                (c . (A))
                (b . (A d end))
                (d . (b))))

(define (walk start end graph [queue '()] [visited '()])
  (if (equal? start end)
      (cons start path)
      (walk 