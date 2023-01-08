# Note taking app for the command line written in Rust
## Fully Explaining each code line to the code , Yes it is messy due to so much comments , but any beginner can take a look at this code and understand 

Notes will be saved in database and its schema will be basically 
id and body column storing the unique identifier and the note's content itself respectively.
It works offline

using sqlite3 as database and rusqlite as the wrapper for it.

rusqlite will be used for the database operations.
the bundled feature tell the package to compile SQLite.

It have CRUD operations for the notes.
- Create ( create a new note, with title, current date and time, and body(when stop writiing just write end in new line and press enter to save the note)) 
- Read ([/list] will list all the saved notes in your database)
- Update ([/edit (id) (changed_body)] will changed the body of the note of the respective Id with the body you provided)
- Delete ([/del (id)] will delete the note)
- clear() (will clear the note saved in buffer_line)
- new (will create a new note, title, date and then you can push the body)
- push (will push the note to the database and wil close the program after pushing)
- end (will end the program without pushing anything) .

## To install in your path so you can use from your command line
```
cargo install --path 
```

## Run the program after installing on your path through 
```
cargo new notesTakingCLI
```

You can modify the name in [Cargo.toml]

Dir notes.db in repo is just the sample .

