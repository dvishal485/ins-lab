#import "assets/front_page.typ": front_page

#front_page
#set page(
  paper: "a4",
  number-align: center,
  margin: 1.27cm,
)

#set text(
  font: "STIX Two Text",
  size: 12pt,
  spacing: 3pt,
  hyphenate: false,
)


#show outline.entry.where(level: 1): it => {
  v(12pt, weak: true)
  strong(it)
}

#outline(indent: 0.6cm)

#set heading(numbering: none)

#let experiments = toml("assets/experiments.toml")

#let codeblock(filename) = {
  set text(size: 11pt)
  let code = read(filename)
  line(length: 100%)
  pad(left:0.25cm)[
    #raw(code, block: true, lang: filename.split(".").at(-1))
  ]
}

#let experiment(n, data) = [
  #pagebreak()
  = Experiment #n

  == Aim
  #data.aim

  == Theory
  #data.theory

  == Program
  #for code in data.program [
    === #code
    #codeblock(code)
  ]
  
  #box(width: 100%)[
    == Output

    #for fig in data.output [
      #figure(
        image(fig.at(1), height: 25%, fit:"contain"),
        caption: fig.at(0),
        supplement: none,
      )
    ]
  ]

  #box(width: 100%)[
    == Conclusion

    #data.conclusion
  ]
]

#for (i, data) in experiments.values().enumerate() {
  [#experiment(i + 1, data)]
}