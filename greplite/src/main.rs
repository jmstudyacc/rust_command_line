fn main() {
    let ctx_lines = 2;
    let query = "mm";
    let source = "\
    Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n\
    sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\
    Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n\
    Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\n\
    Excepteur sint occaecat cupidatat non proident,\n\
    sunt in culpa qui officia deserunt mollit anim id est laborum.";

    // holds the line numbers where matches occur
    let mut tags: Vec<usize> = vec![];

    // contains a vector per match to hold the context lines
    let mut ctx : Vec<Vec<(usize, String)>> = vec![];

    // iterates through the lines recording line numbers where matches are encountered
    for ( i, line ) in source.lines().enumerate()
    {
        if line.contains(query)
        {
            // if there is a match then push the result into the tags Vec
            tags.push(i);

            // a Vec::with_capacity(n) reserves space for 'n' items - type inference occurs for this
            let v = Vec::with_capacity(2*ctx_lines + 1);
            ctx.push(v);
        }
    }

    // if there are no matches then exit the program early
    if tags.is_empty()
    {
        return;
    }

    // for each line
    for ( i, line ) in source.lines().enumerate()
    {
        // for each tag
        for ( j, tag ) in tags.iter().enumerate()
        {
            // saturating_sub() returns 0 integer on an integer underflow to avoid panic
            let lower_bound = tag.saturating_sub(ctx_lines);
            // value of tag + number of context lines for the upper bound
            let upper_bound = tag + ctx_lines;

            // if i is lte upper && gte lower then...
            if ( i >= lower_bound ) && ( i <= upper_bound )
            {
                // create a String from the current line
                let line_as_string = String::from(line);

                // local context is value of i and the new String
                let local_ctx = ( i, line_as_string );

                // push this tuple value onto ctx Vec at index of value j
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter()
    {
        // ref line, informs the compiler that we want to borrow the value, not move it
        for &( i, ref line ) in local_ctx.iter()
        {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
