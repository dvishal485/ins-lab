#let front_page = [
  #set page(
    paper: "a4",
    number-align: center,
    margin: 2.5cm,
  )

  #set text(
    font: "STIX Two Text",
    size: 14pt,
    hyphenate: false,
  )

  #set par(leading: 25pt)


  #align(
    center,
    [
      #heading(outlined: false)[ 
        #smallcaps("Information and Network Security")
      ]

      #v(1cm)

      #heading("Lab File (CO425)", outlined: false, bookmarked: false, depth: 2)

      #v(4cm)

      #image("images/dtu_logo.png")
      #v(1cm)

      #text([*Department of Computer Science*], size: 18pt)


      Delhi Technological University

      #text(
        [
          (Formerly, Delhi College of Engineering)

          Bawana Road, Delhi-110042
        ],
        size: 12pt,
      )

    ],
  )

  #v(2cm)

  #align(left)[
    *Submitted by*

    Vishal Das
    #v(0.5cm)

    *Submitted to*

    INS Teacher
  ]
]