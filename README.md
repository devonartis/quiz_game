# Exercise #1: Quiz Game


**## Excercise details**

This excercise was taken from [Gophercise Excercise 1](https://courses.calhoun.io/lessons/les_goph_01)



*Part 1*

Create a program that will read in a quiz provided via a CSV file (more details below) and will then give the quiz to a user keeping track of how many questions they get right and how many they get incorrect. Regardless of whether the answer is correct or wrong the next question should be asked immediately afterwards.

The CSV file should default to problems.csv (example shown below), but the user should be able to customize the filename via a flag.

The CSV file will be in a format like below, where the first column is a question and the second column in the same row is the answer to that question.


### What I learned 



```
/*
Use the csv::ReaderBuilder when reading csv files With no headers an error occurs:

Error: Error(Deserialize { pos: Some(Position { byte: 9, line: 2, record: 1 }), err: DeserializeError { field: None, kind: Message("missing field question") } })
    
    
*/
