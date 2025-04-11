# Find all Rust files
Get-ChildItem -Path src -Filter "*.rs" -Recurse | ForEach-Object {
    $file = $_.FullName
    Write-Host "Processing $file"
    
    # Read the file content
    $content = Get-Content -Path $file -Raw
    
    # Replace 'use gtk::prelude::*;' with 'use crate::prelude::*;'
    $content = $content -replace 'use gtk::prelude::\*;', 'use crate::prelude::*;'
    
    # Replace 'use gtk4::prelude::*;' with 'use crate::prelude::*;'
    $content = $content -replace 'use gtk4::prelude::\*;', 'use crate::prelude::*;'
    
    # Replace 'use gtk;' with 'use crate::prelude::*;'
    $content = $content -replace 'use gtk;', 'use crate::prelude::*;'
    
    # Replace standalone 'use gtk' followed by imports with prelude
    $content = $content -replace 'use gtk::\{', 'use crate::prelude::{'
    
    # Replace 'use gtk4::{' with 'use crate::prelude::{' 
    $content = $content -replace 'use gtk4::\{', 'use crate::prelude::{'
    
    # Write the file back
    Set-Content -Path $file -Value $content
}

Write-Host "All files processed." 