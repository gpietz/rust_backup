# RustBackup

Maybe I'm old fashioned but besides GitHub I prefer to store the source code of my projects on disk. Especially for
prototyping I find this handy, as I don't like to upload unfinished states to the web. 

This small Rust project I wanted to write for a long time and I started it the days before. Since I am still relatively 
new in the Rust environment, I could learn a lot from it. But what is it for? 

It's a CLI tool which helps to pack complete projects into zip files considering the project structure and the .gitignore files. 
This means that only the files that are necessary to create the executable files are included in the archive. 
