# Exercise #1: Quiz Game


**## Excercise details**


This exercise is broken into two parts to help simplify the process of explaining it as well as to make it easier to solve. The second part is harder than the first, so if you get stuck feel free to move on to another problem then come back to part 2 later.

Note: I didn’t break this into multiple exercises like I do for some exercises because both of these combined should only take ~30m to cover in screencasts.

*Part 1*

Create a program that will read in a quiz provided via a CSV file (more details below) and will then give the quiz to a user keeping track of how many questions they get right and how many they get incorrect. Regardless of whether the answer is correct or wrong the next question should be asked immediately afterwards.

The CSV file should default to problems.csv (example shown below), but the user should be able to customize the filename via a flag.

The CSV file will be in a format like below, where the first column is a question and the second column in the same row is the answer to that question.



```
/*
Use the csv::ReaderBuilder when reading csv files With no headers an error occurs:
 
Error: Error(Deserialize { pos: Some(Position { byte: 9, line: 2, record: 1 }), err: DeserializeError { field: None, kind: Message("missing field question") } })
    
    
*/
