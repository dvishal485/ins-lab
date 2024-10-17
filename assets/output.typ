#import "term.typ": term

#let draw_terminal(fig) = [
  #set text(size: 12pt)
  #term(
    content: read(fig.at(1)),
    command: "dvishal485@tornado:~/Projects/ins$ cargo run -r --example " + fig.at(1).rev().split(".").at(-1).split("/").at(0).rev(),
  )

  #align(center, text(size: 10pt, fig.at(0)))
]