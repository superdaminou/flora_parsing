# Flora's parser
A floral parser for specific research MED-PC files. 

This application is under construction, for any problem please create an issue. I'm no scientist, just someone with strange hobbies. 

## Basic Usage
Get the appropriate release: https://github.com/superdaminou/flora_parsing/releases

## Modes 
For now there is three mode availables:

#### csv 
Simple parsing to ease manipulation and produce something similar to csv content.

```
A: 
Action;Time;
000;5;
000;60;
000;30;
```

#### action_time
Calculate the sum of each actions occurence at each time for every group;
```
Time 7381:  
Actions: 500, occurences: 21
Actions: 200, occurences: 12
Actions: 310, occurences: 1
Actions: 600, occurences: 20
Actions: 100, occurences: 267
```

#### total_action
Calculate the total elapsed time for each action for every group
```
C: 
Action: 100, Total Time: 329663
Action: 500, Total Time: 0
Action: 200, Total Time: 0
Action: 600, Total Time: 22956
Action: 310, Total Time: 7381
```
