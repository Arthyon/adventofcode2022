open System.Collections.Generic
open System.IO
open System.Text.RegularExpressions

let transpose matrix =
         matrix
         |> Seq.collect Seq.indexed
         |> Seq.groupBy fst
         |> Seq.map (snd >> Seq.map snd)

let createStack (row: char seq) =
    match row |> Seq.rev |> Seq.toList with
    | head::tail when System.Char.IsNumber head ->
        Some(Stack(tail |> Seq.filter (System.Char.IsWhiteSpace >> not)))
    | _ -> None
    
let rec createStacks (lines: string list) =
    let matrix = lines |> Seq.map (fun x -> x.ToCharArray())
    matrix
         |> transpose
         |> Seq.choose createStack
         |> Seq.toList
    
let separateStackAndCommands (lines: string list) =
    let idx = lines |> Seq.findIndex System.String.IsNullOrWhiteSpace
    let stack, commands = lines |> List.splitAt idx
    stack |> createStacks, commands[1..]
    
let parseCommand cmd =
    let m = Regex(@"move (\d+) from (\d+) to (\d+)").Match cmd
    int m.Groups[1].Value, int m.Groups[2].Value, int m.Groups[3].Value
    
let applyCommands mover (stacks: Stack<char> list, commands: string seq) =
    for cmd in commands do
        let no, source, target = parseCommand cmd
        let sourceStack = stacks[source - 1]
        let targetStack = stacks[target - 1]
        mover no sourceStack targetStack
    stacks
    
let CrateMover9000 count (source: Stack<char>) (target: Stack<char>) =
    for i = 1 to count do
        let v = source.Pop()
        target.Push(v)
            
let CrateMover9001 count (source: Stack<char>) (target: Stack<char>) =
    let l = List<char>()
    for i = 1 to count do
        l.Add(source.Pop())

    for item in l |> Seq.rev do target.Push(item)
let print (stack: Stack<char>) = printf "%c" <| stack.Pop()

let input = File.ReadLines("input")
input |> Seq.toList |> separateStackAndCommands |> applyCommands CrateMover9000 |> Seq.iter print
printfn ""
input |> Seq.toList |> separateStackAndCommands |> applyCommands CrateMover9001 |> Seq.iter print

