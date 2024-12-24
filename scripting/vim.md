# Best text editor - Vim

- There are three modes: insert, command and last-line;
- When file is opened via Vim, it is started in command mode; So when typing **j**, it will move cursor down by one line, rather than inserting **j**;
- Type **i** to get into insert mode and hiy escape to exit the mode;
- Press **:** to switch to the last line mode;

## Moving in Vim

- use **w** to jump one word forward and **b** to jump one word backward;
- use **j** to move cursor down one line and **k** to jump one line up;
- use **G** to move to the end of the file and **gg** to move to the beginning of the file;
- use **$** to move cursor to the end of the line and **0** to move to the start of the line;

**Important!!**
Use prefacing numbers and Vim will execute this command multiple times;

- For example, move 5 words forward with **5w**, to move bakc 10 words use **10b**, to move down 2 lines use **2j** and to move 3 lines up use **3k**.

## Editing mode in Vim (not in insert mode)

- use **u** to undo an insert operation! To undo your last 'undo' hit Ctrl-r;
- use **dw** to delete a word , **d0** to delete to the beginning of the line, **d$** to delete to the end of the line;
- use global **dgg** to delete to the beginning of the file and **dG** to delete to the end of the file;
- use **2dd** to delete two lines below the cursor;
- use **/** to search for a certain word in the text; Then hit **n** tojump to the next occurence;
- use **v** to copy one chacter at a time and **V** to highlight a whole line. Use **p** to paste it after the current line and **P** to paste it on the current line;
