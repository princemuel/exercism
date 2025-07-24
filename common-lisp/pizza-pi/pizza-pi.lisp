(defpackage :pizza-pi
  (:use :cl)
  (:export :dough-calculator :pizzas-per-cube
           :size-from-sauce :fair-share-p))

(in-package :pizza-pi)

(defun dough-calculator (pizzas diameter)
  "Calculates the amount of dough needed for given number and size of pizzas"
  (round ( * pizzas (+ (/ (* 45 pi diameter) 20) 200))))

(defun size-from-sauce (sauce))

(defun pizzas-per-cube (cube-size diameter))

(defun fair-share-p (pizzas friends))
