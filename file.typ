#import "assets/front_page.typ": front_page

#let file_data = toml("assets/experiments.toml")

#set page(
  paper: "a4",
  number-align: center,
  margin: (left: 1.7cm, y: 1.27cm, right: (1.27cm * 2 - 1.27cm)),
  footer: text(size: 10pt)[
    #file_data.info.student
    #if file_data.info.roll_number.len() > 0 [ (#file_data.info.roll_number) ]
    #h(1fr)
    _#context counter(page).get().at(0)_
  ],
  background: rect(
    width: 100% - 1cm,
    height: 100% - 0.75cm,
    radius: 0.1cm,
  ),
)

#front_page(
  student_name: file_data.info.student,
  subject_name: file_data.info.subject_name,
  teacher_name: file_data.info.professor.name, 
  file_heading: file_data.info.file_heading, 
  logo: file_data.info.logo,
  college_name: file_data.info.college.name, 
  college_address: file_data.info.college.address,
  department: file_data.info.college.department,
  roll_number: file_data.info.roll_number,
  professor_designation: "Assistant Professor"
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

#let codeblock(filename) = {
  set text(size: 11pt)
  let code = read(filename)
  line(length: 100%)
  block(fill: luma(97%), width: 100%, radius: 0.20cm)[
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

#let exp_list = file_data.keys().filter(n => n.starts-with("experiment")).map(n => int(n.split("experiment").at(1)))

#for (i, data) in exp_list.enumerate() {
  [#experiment(i + 1, file_data.at("experiment" + str(data)))]
}