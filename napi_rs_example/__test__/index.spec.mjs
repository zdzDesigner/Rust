import test from "ava"

import { sum, Animal, callThreadsafeFunction, listen } from "../index.js"

console.log({ Animal, callThreadsafeFunction })

test("sum from native", (t) => {
  t.is(sum(1, 2), 3)
})

test("thread safe:", (t) => {
  callThreadsafeFunction((...args) => {
    // console.log({ args })
  })
  t.is(true, true)
})

test("listen:", (t) => {
  listen("read", (...args) => {
    console.log({ args })
  })
  t.is(true, true)
})
