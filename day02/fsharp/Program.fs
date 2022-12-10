open System.IO

type Choice =
    | Rock = 1
    | Paper = 2
    | Scissors = 3
    
type Outcome =
    | Win = 6
    | Lose = 0
    | Draw = 3
    
let fight = function
    | Choice.Rock, Choice.Rock
    | Choice.Paper, Choice.Paper
    | Choice.Scissors, Choice.Scissors -> Outcome.Draw
    | Choice.Rock, Choice.Scissors 
    | Choice.Paper, Choice.Rock 
    | Choice.Scissors, Choice.Paper -> Outcome.Win 
    | Choice.Rock, Choice.Paper 
    | Choice.Paper, Choice.Scissors 
    | Choice.Scissors, Choice.Rock -> Outcome.Lose
    | _ -> failwith "Invalid enum value"
    
let calculateScore (myChoice, opponentChoice) =
    let outcome = fight (myChoice, opponentChoice)
    int myChoice + int outcome

let parseCharacter = function
    | "A" | "X" -> Choice.Rock
    | "B" | "Y" -> Choice.Paper
    | "C" | "Z" -> Choice.Scissors
    | _ -> failwith "Invalid character"
let parseOutcome = function
    | "X" -> Outcome.Lose
    | "Y" -> Outcome.Draw
    | "Z" -> Outcome.Win
    | _ -> failwith "Invalid character"
let chooseHand = function
    | Choice.Rock, Outcome.Win -> Choice.Paper
    | Choice.Rock, Outcome.Lose -> Choice.Scissors
    | Choice.Rock, Outcome.Draw -> Choice.Rock
    | Choice.Paper, Outcome.Win -> Choice.Scissors
    | Choice.Paper, Outcome.Lose -> Choice.Rock
    | Choice.Paper, Outcome.Draw -> Choice.Paper
    | Choice.Scissors, Outcome.Win -> Choice.Rock
    | Choice.Scissors, Outcome.Lose -> Choice.Paper
    | Choice.Scissors, Outcome.Draw -> Choice.Scissors
    | _ -> failwith "Invalid hand"
    
let parse parser (line: string) =
    match line.Split(' ') with
    | [|first;second|] -> parser (first, second)
    | _ -> failwith "Invalid data"
    
let part1Parser (first, second) = (parseCharacter second, parseCharacter first)
let part2Parser (first, second) =
    let opponentHand = parseCharacter first
    let outcome = parseOutcome second
    (chooseHand (opponentHand, outcome), opponentHand)

let score = File.ReadAllLines("input")
            |> Seq.map (parse part2Parser)
            |> Seq.map calculateScore
            |> Seq.sum
printf $"%A{score}"