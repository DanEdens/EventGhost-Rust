#!/bin/bash

# Find all Rust files
find src -name "*.rs" | while read file; do
  echo "Processing $file"
  
  # Replace 'use gtk::prelude::*;' with 'use crate::prelude::*;'
  sed -i 's/use gtk::prelude::\*;/use crate::prelude::\*;/g' "$file"
  
  # Replace 'use gtk4::prelude::*;' with 'use crate::prelude::*;'
  sed -i 's/use gtk4::prelude::\*;/use crate::prelude::\*;/g' "$file"
  
  # Replace 'use gtk;' with 'use crate::prelude::*;'
  sed -i 's/use gtk;/use crate::prelude::\*;/g' "$file"
  
  # Replace standalone 'use gtk' followed by imports with prelude
  sed -i 's/use gtk::{/use crate::prelude::{/g' "$file"
  
  # Replace 'use gtk4::{' with 'use crate::prelude::{' 
  sed -i 's/use gtk4::{/use crate::prelude::{/g' "$file"
  
  # Replace 'use gtk::(MODULE)' with 'use crate::prelude::(MODULE)'
  sed -i 's/use gtk::\([a-zA-Z]*\)/use crate::prelude::\1/g' "$file"
done

echo "All files processed." 