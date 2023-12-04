(ns aoc2
  (:require [clojure.java.io :as io]
            [clojure.string :as str]))

(def example
  (slurp (io/resource "example")))

(def input
  (slurp (io/resource "02.input")))

(defn parse-cube
  "ex: 3 blue -> {blue 3}"
  [cube]
  (let [[count color] (str/split cube #" ")]
    {color (Integer/parseInt count)}))

(parse-cube "3 blue")

(defn parse-cube-set
  "ex: 3 blue, 4 red -> {blue 3, red 4}"
  [cube-set]
  (let [splitted (-> cube-set (str/split #","))]
    (->> splitted
         (map str/trim)
         (map parse-cube)
         (reduce merge))))

(parse-cube-set "3 blue, 4 red")

(defn parse-line [line]
  (let [[game record] (-> line (str/split #":"))
        game-number (-> game (str/split #" ") second)
        record-info (-> record (str/split #";"))
        sets (->> record-info (map parse-cube-set))]
    {:game game-number :sets sets}))

(->
 (parse-line "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
 (:sets))

(defn cube-set-possible? [cube-set schema]
  (let [colors (keys cube-set)]
    (every? (fn [color] (<= (get cube-set color 0) (get schema color 0))) colors)))

(keys (parse-cube-set "3 blue, 4 red"))

(cube-set-possible?
 (parse-cube-set "3 blue, 4 red")
 {"red" 10 "green" 0 "blue" 1})

(defn possible?
  [game schema]
  (every? #(cube-set-possible? % schema) (:sets game)))

(possible?
 (parse-line "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
 {"red" 12 "green" 13 "blue" 14})

(def schema {"red" 12 "green" 13 "blue" 14})

;; part1
(->> input
    str/split-lines
    (map parse-line)
    (filter #(possible? % schema))
    (map #(get % :game))
    (map #(Integer/parseInt %))
    (reduce +))
