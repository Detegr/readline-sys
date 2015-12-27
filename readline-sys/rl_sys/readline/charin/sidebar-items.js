initSidebarItems({"fn":[["clear_pending_input","Unset `rl_pending_input`, effectively negating the effect of any previous call to `rl_execute_next()`. This works only if the pending input has not already been read with `rl_read_key()`."],["execute_next","Make `c` be the next command to be executed when `rl_read_key()`` is called. This sets `rl_pending_input`."],["getc","Return the next character available from `stream`, which is assumed to be the keyboard."],["read_key","Return the next character available from Readline's current input stream. This handles input inserted into the input stream via `rl_pending_input` (see section [2.3 Readline Variables]) and `rl_stuff_char()`, macros, and characters read from the keyboard. While waiting for input, this function will call any function assigned to the `rl_event_hook` variable. [2.3 readline variables]: https://goo.gl/E1D6om"],["set_keyboard_input_timeout","While waiting for keyboard input in `rl_read_key()`, Readline will wait for `us` microseconds for input before calling any function assigned to `rl_event_hook`. `us` must be greater than or equal to zero (a zero-length timeout is equivalent to a poll). The default waiting period is one-tenth of a second. Returns the old timeout value."],["stuff_char","Insert `c` into the Readline input stream. It will be \"read\" before Readline attempts to read characters from the terminal with `rl_read_key()`. Up to 512 characters may be pushed back. `rl_stuff_char` returns 1 if the character was successfully inserted; 0 otherwise."]]});