$ 1abc2
$ pqr3stu8vwx
$ a1b2c3d4e5f
$ treb7uchet
ExI ←

$ two1nine
$ eightwothree
$ abcone2threexyz
$ xtwone3four
$ 4nineeightseven2
$ zoneight234
$ 7pqrstsixteen
ExII ←

In ← ⍣(&fras"1.in") ""
Lines ← ⊜□≠@\n.

Numbers ← {
  "zero" "one" "two" "three" "four" "five"
  "six" "seven" "eight" "nine"}

Solve! ← /+≡◇(⋕⊂⊃(⊢|⊢⇌) ^!) Lines

I ← Solve!(▽<@a.)

MatchOne ← ⊢♭regex/$"_|_"
MatchFirst ← MatchOne ⊂Numbers{"\\d"}
MatchLast ← ⍜⇌(MatchOne ⊂≡⇌Numbers{"\\d"})
II ← Solve!(+@0≡⍣(⋕|⊗:Numbers) ⊂⊃(MatchFirst|MatchLast))

I ExI
I In
II ExII
II In
