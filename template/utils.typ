
#let collapsing_list(items) = {
  if items.len() > 3 {
    set text(hyphenate: true)
    items.join(", ")
  } else {
    items.join(linebreak())
  }
}

#let shrink_text_to_width(content, max) = context {
    let measured_width = measure(content).width
    let maximum_width = max.cm()
    let font_size = text.size

    while measured_width.cm() > maximum_width {
        font_size -= 0.1pt
        measured_width = measure(text(size: font_size)[#content]).width
    }

    text(size: font_size)[#content]
}


#let shrink_text_to_height(target_height, content) = layout(size => {
  let font_size = text.size
  
  let (height,) = measure(
    block(width: size.width, text(size: font_size)[#content]),
  )
  
  while height > target_height {
      font_size -= 0.1pt
      height = measure(
        block(width: size.width, text(size: font_size)[#content]),
      ).height
  }
  
  block(
      height: height,
      width: 100%,
      text(size: font_size)[#content]
  )
})