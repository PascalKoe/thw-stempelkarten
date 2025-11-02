#import "@local/tiaoma:0.3.0"
#import "utils.typ": *


#set page(
  margin: 8mm
)


#set text(font: "BundesSans Web", size: 12pt)

#let picture(picture_file) = {
  image(picture_file, height: 25mm)
};

#let qualified_indicator(qualified, hidden: false) = {
  if not hidden {
    let fill = if qualified {
      rgb(0, 255, 0)
    } else {
      rgb(255, 0, 0)
    }
  
    circle(
      width: 5mm,
      height: 5mm,
      fill: fill,
    )
  }
}

#let name_section(last_name, first_name) = layout(ctx => {
  set par(leading: 5pt)
  set text(size: 12pt)

  [
    #set text(weight: "bold")
    #shrink_text_to_width(last_name + ",", ctx.width)\
    #set text(weight: "regular")
    #shrink_text_to_width(first_name, ctx.width)\
  ]
})

#let person_column(last_name, first_name, qualified, hide_qualified, picture_file) = {
  let content = [
    #place(top + right, qualified_indicator(qualified, hidden: hide_qualified))
    #picture(picture_file)
    #name_section(last_name, first_name)
  ]
  
  content
}

#let qualifications_section(qualifications) = {
  let qualifications = qualifications.sorted();
  
  if qualifications.len() > 3 {
    set text(hyphenate: true)
    qualifications.join(", ")
  } else {
    qualifications.join(linebreak())
  }
}

#let details_column(deployment, licenses, qualifications) = layout(size =>{
  set par(leading: 4pt, spacing: 0mm)
  set text(weight: "regular", size: 10pt, ligatures: true, kerning: true)

  // upper section containing the deployment information
  let upper_section = [
    #collapsing_list(deployment)
    #v(5mm)
  ];

  // lower section for which the font is lowered until everything fits in
  let lower_section_height = size.height - measure(upper_section).height;
  let lower_section = [
    #if licenses.len() > 0 {
      collapsing_list(licenses.sorted())
      v(5mm)
    }
    #collapsing_list(qualifications.sorted())
  ];

  // combined lower and upper section
  [
    #upper_section
    #shrink_text_to_height(lower_section_height, lower_section)
  ]
})


#let card(last_name, first_name, qualified, deployment, licenses, qualifications, registration_id, picture_file, hide_qualified) = {
  set box(width: 100%, height: 100%)
  
  let person_container = box(
    inset: (top: 3mm, left: 3mm, bottom: 3mm, right: 3mm),
    person_column(last_name, first_name, qualified, hide_qualified, picture_file)
  );

  let details_container = box(
    inset: (top: 3mm, left: 0mm, bottom: 3mm, right: 0mm),
    details_column(deployment, licenses, qualifications)
  );

  let barcode_container =  box(
    inset: (top: 3mm, left: 3mm, bottom: 3mm, right: 3mm),
    rotate(reflow: true, 270deg, tiaoma.code128(
      height: 100%,
      width: 100%,
      options: (show-hrt: false),
      registration_id
    )),
  );

  block(
    width: 95mm,
    height: 48mm,
    // stroke: 1pt + gray,
    grid(
      columns: (40mm, auto, 18mm),
      rows: (100%),
      
      person_container,
      details_container,
      barcode_container
    )
  )
}

#let input = sys.inputs.volunteers
#for chunk in input.chunks(10) {
  let cards = chunk.map(details => {
    card(details.last_name, details.first_name, details.qualified, details.deployment, details.licenses, details.qualifications, details.barcode, details.picture, details.hide_qualified)
  })

  [
    #grid(
      columns: 2,
      stroke: black + 1pt,
      ..cards
    )
    #pagebreak(weak: true)
  ]
}