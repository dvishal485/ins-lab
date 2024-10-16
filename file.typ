#import "assets/front_page.typ": front_page

#front_page

#set page(
  paper: "a4",
  number-align: center,
  margin: 1.27cm,
  footer: text(size: 10pt)[
    Vishal Das #h(1fr) #context counter(page).get().at(0)
  ],
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
    block(fill: luma(97%), width: 100%)[
      #pad(x: 0.25cm, y: 0.25cm)[
        #raw(code, block: true, lang: filename.split(".").at(-1))
      ]
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
        image(fig.at(1), height: 25%, fit: "contain"),
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

#let sorted_list = experiments.keys().map(n => int(n.split("experiment").at(1)))

#for (i, data) in sorted_list.enumerate() {
  [#experiment(i + 1, experiments.at("experiment" + str(data)))]
}