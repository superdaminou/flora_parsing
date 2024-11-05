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
C: 
Time;Action;Occurences;
0;;
  ;500;1;
2521;;
  ;500;1;
  ;100;1;
```

#### Action by time
Calculate the sum of each actions occurence at each time for every group;
```
Time 7381:  
Actions: 500, occurences: 21
Actions: 200, occurences: 12
Actions: 310, occurences: 1
Actions: 600, occurences: 20
Actions: 100, occurences: 267
```

#### Total time by action
Calculate the total elapsed time for each action for every group
```
C: 
Action;Total Time;
500;0;
310;7381;
600;22956;
100;329663;
200;0;
```
