use rusqlite::{Connection, Result};

// returns a Result type which is an enum with Ok and Err variants
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a connection to the database, if the database doesn't exist, it will be created
    // open is a static method of Connection struct, notes.db is the name of the database file
    let conn = Connection::open("notes.db")?; // ? is used to return the error from a function as it is

    // create a table named notes if it doesn't exist
    // execute comes from the Connection struct which do is execute a SQL statement
    // the first argument is the SQL statement, the second argument is a slice of values to be bound to the statement
    // here id is the primary key, title and body are not null, created_at is not null
    // text not null means that the column can't be null and the value must be a text
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (  /* // IF NOT EXISTS means that the table will be created only if it doesn't exist */
                  id              INTEGER PRIMARY KEY,
                  title           TEXT NOT NULL,
                  body            TEXT NOT NULL,
                  created_at      TEXT NOT NULL
                  )",
        [],  // here we are passing an empty slice because we don't have any values to be bound to the statement 
        // if we had values to be bound to the statement, we would pass them as a slice like this:
        // [1, "hello", "world", "2021-01-01 00:00:00"] // the order of the values must be the same as the order of the columns in the table
    )?;

    // insert a new note into the notes tableq

    // input for title and date
    let mut title = String::new();
    let mut body = String::new();
    let mut created_at = String::new();


    // get the body in a loop
    let mut running = true; // a boolean variable to control the loop, if it's true, the loop will continue,
    // if it's false, the loop will stop
    while running == true {
        // while the running variable is true, the loop will continue
        let mut line = String::new(); // create a new string
        std::io::stdin().read_line(&mut line)?; // read a line from the user and store it in the line variable

        // functionality for deleting the notes by Id number as argument , use /del 
        // same functionality applies for /edit a certain note by id
        // As /del command takes additional parameter (the id of the note) , In order to differentiate b/w command 
        // and parameter , split_once method splits a string on the basis of passsed delimeter
        let trimmed_body = line.trim();  // trimming the line here for anywhitespace in both side of the string
        let cmd_split = trimmed_body.split_once(" ");  //" " means If there is space between string
        // split those string into two .
        //split_once method returns two &str by splitting one string basis of delimeter
        // example "/del 1" would return as Some(("/del", "1")).
        let mut cmd = trimmed_body; 
        let mut msg = "";
        // if cmd is equals to /del then we will run that else if condition 
        if cmd_split != None { // If delimeter exists, if not exists than it would be equals to None
            cmd = cmd_split.unwrap().0; // saving the first &str which is /del or /edit in cmd
            msg = cmd_split.unwrap().1; // and second &str which is the id in msg variable
            // when /edit we will pass it like this 
            // "/edit 1 this is the new body"
            // so cmd will be save to "/edit"
            // but msg will be save to "1 this is the new body"
            // and then we split it msg once again in else if condition into id and body argument.
        }


        if line.trim() == "new" {
            // get the title
            println!("Enter the title of the note: ");
            std::io::stdin().read_line(&mut title)?; // ? is used to return the error from a function as it is

            //get the date automatic
            // chrono is a crate for date and time handling
            // local is a struct that represents the local date and time
            // now is a static method of the local struct that returns the current date and time

            let now = chrono::Local::now(); // get the current date and time

            // format is a method of the DateTime struct that formats the date and time
            created_at = now.format("%Y-%m-%d %H:%M:%S").to_string(); // format the date and time
            line.clear();
            println!("Enter the body of the note: "); // print a message to the user
            std::io::stdin().read_line(&mut body)?;
            body.push_str(&line); // if the user didn't enter "end", the line will be added to the body variable and the loop will continue
        } else if line.trim() == "end" {
            // if the user entered "end", the loop will stop
            running = false;
        } else if line.trim() == "/list" {
            // clearing the line
            line.clear();
            // if the user entered "/list", the list of notes will be printed all the notes in the notes table database
            // with everything in the table

            // conn.prepare means that the SQL statement will be prepared and then executed multiple times with different parameters
            // SELECT * FROM notes means that all the columns in the notes table will be selected
            let mut stmt = conn.prepare("SELECT * FROM notes")?; // Select all the columns in the notes table

            // stmt.query means that the SQL statement will be executed and the result will be returned as a Result<Rows, Error>
            // here query stands for query the database
            // rusqlite::params![]
            let mut rows = stmt.query(rusqlite::params![])?; // params![] is a macro that creates an empty slice of values to be bound to the statement
             // as we are selecting all the columns in the notes table, we don't have any values to be bound to the statement
             // if there were values to be bound to the statement, we would pass them as a slice like this:
             // rusqlite::params![1, "hello", "world", "2021-01-01 00:00:00"] // the order of the values must be the same as the order of the columns in the table
            while let Some(row) = rows.next()? {
                // loop through the rows returned by the query
                // let Some(row) means that the loop will continue as long as there are rows in the rows variable
                // if there are no more rows, the loop will stop
                // rows.next()? means that the next row will be returned as a Result<Row, Error>

                // rust cannot infer the types of these properties which is why they mist be specified
                let id: i32 = row.get(0)?; // get the value of the first column in the row and store it in the id variable
                let title: String = row.get(1)?;
                let body: String = row.get(2)?;
                let created_at: String = row.get(3)?;
                // print each row on a new line
                println!(
                    "ID:{} Title:{} 
                    
Body:{} 

Date:{} || 
",
                    id,
                    title.to_string().trim(),
                    body.to_string().trim(),
                    created_at.to_string().trim()
                );
            }
            // push the note into the database when the user enters "push"
        } else if line.trim() == "push" {
            conn.execute(
                "INSERT INTO notes (title, body, created_at) VALUES (?1, ?2, ?3)",
                &[&title, &body, &created_at], // the order of the values must be the same as the order of the columns in the table
                // & in &[] means that the values are references to the variables
            )?;
            println!("Note pushed successfully");
            running = false;
        } else if cmd == "/del" {
            let id = msg;
            // here (?1) is showing that id number starts from 1 and wents to 32766 in sqlite table, 
            // so we have to put the bare minimum atleast
            // This is the SQL command to delete a row from the notes table that matches the id specified.
            conn.execute("delete from notes where id = (?1)", [id])?; // passing id as the param as we have to
            // that note associated with that id


        } else if cmd == "/edit" {
            // split_once returns tuple of split strings
            //  The main difference is we need to again split the msg by a white space.
            // Using the split_once only splits on the first white space, which allows the body to remain intact.
            let (id, body) = msg.split_once(" ").unwrap();
            // (?1) and (2?) are positional parameters , as body is 1 so its ?1 and id is at second param so its ?2,
            // this command will set the body of the body param
            conn.execute("update notes set body = (?1) where id = (?2)", [body, id])?;
        }
        // clearing the line variable
        else if line.trim() == "clear()" {
            line.clear();
        }
        else {
            // keep opened the command line
            body.push_str(&line);
        }
    }

    /*  // insert the note into the notes table
    conn.execute(
        "INSERT INTO notes (title, body, created_at) VALUES (?1, ?2, ?3)",
        &[&title, &body, &created_at], // the order of the values must be the same as the order of the columns in the table
        // & in &[] means that the values are references to the variables
    )?; */

    Ok(())
}

// Result<(), Box<dyn std::error::Error>> is a type alias for Result<T, E> where T is () and E is Box<dyn std::error::Error>
// the () type is the unit type and has a single value, also written as (),
// basically the main function is returning result which will be completed successfully with no value or fails with an error.

// ok(()) is showing that if the main function is successful, it will return Ok(()) which is a unit type.
// basically nothing is returned, and ? is handling the error where the function will return the error if it fails.

// the ? operator is used to return the error from a function as it is. It basically works like this:
// fn foo() -> Result<i32, io::Error> {
//     let f = File::open("bar.txt")?;
//     Ok(0)
// }
// is equivalent to:
// fn foo() -> Result<i32, io::Error> {
//     let f = match File::open("bar.txt") {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     Ok(0)
// }

// Box means that the type is allocated on the heap and a pointer to the allocated memory is stored on the stack.
// dyn means that the type is a trait object.
// std::error::Error is a trait that is implemented by the Error trait for the error types in the standard library.
// Box<dyn std::error::Error> is a trait object for the Error trait, meaning it's a trait object for any type that implements the Error trait.
// it is working like this:
// fn foo() -> Result<i32, Box<dyn Error>> {
//     let f = File::open("bar.txt")?;
//     Ok(0)
// }
// is equivalent to:
// fn foo() -> Result<i32, Box<dyn Error>> {
//     let f = match File::open("bar.txt") {
//         Ok(file) => file,
//         Err(e) => return Err(Box::new(e)),
//     };
//     Ok(0)
// }

// why we are using Box here? because we don't know the exact type of the error that will be returned,
// so we are using Box to return a trait object for any type that implements the Error trait.
// but we are not defining our custom error type yet, so we are using Box<dyn std::error::Error> for now.
