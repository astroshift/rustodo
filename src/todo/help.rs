pub fn help() {
    println!(r#"Usage: todo [OPTION...] [-a "item to add"] [-f "string in item"] [-d "string in item"] [-r "string in item"]

todo - Save all the various items you need to do in one place

optional arguments:

 -a "item"    add an item to your todo list

 -f "query"   finish an item(s) on your todo list
 -u "query"   unfinish an item on your todo list

 -r "query"   remove an item(s) from your todo list, looks for shortened names be careful if similar items

 -c         clear your todo list, asks to make sure
 
 -l         list everything on your todo list
 
 -h         show this help message


Program misuse is a result of not adding enough arguments to an option such as -a "item" 
Simply include the item or query you would like to add/remove/finish in quotes to proceed"#);
}
