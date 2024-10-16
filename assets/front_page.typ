#let front_page(student_name: "John Doe", subject_name: "INS", teacher_name: "Professor", file_heading: "Lab File", logo: "image.png" , college_name : "College", college_address: "India", department: "Department of Computer Science") = [
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
        #smallcaps(subject_name)
      ]

      #v(1cm)

      #heading(file_heading, outlined: false, bookmarked: false, depth: 2)

      #v(4cm)

      #image(logo)
      #v(1cm)

      #text([*#department*], size: 18pt)


      #college_name

      #text(
        college_address,
        size: 12pt,
      )

    ],
  )

  #v(2cm)

  #align(left)[
    *Submitted by*

    #student_name
    #v(0.5cm)

    *Submitted to*

    #teacher_name
  ]
]