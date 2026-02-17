gtime -v cairo-prove prove target/release/cairo_tee_replacement.executable.json ./proof.json

gtime -v cairo-prove verify proof.json   
