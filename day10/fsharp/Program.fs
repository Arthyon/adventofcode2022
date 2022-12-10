open System.IO

type Command = | Noop | Add of int

let parse (line: string) = seq { Noop; if line.StartsWith("addx") then Add(line.Substring(5) |> int) }

let execute register = function
    | Add x -> register + x
    | Noop -> register

let part1 input =
    let monitorCycle x cycle = if [20; 60; 100; 140; 180; 220] |> Seq.contains cycle then x * cycle else 0
    
    let rec part1' cycle x sum = function
        | head::tail -> let sum = sum + (monitorCycle x <| cycle + 1)
                        part1' (cycle + 1) (execute x head) sum tail
        | [] -> sum
        
    part1' 0 1 0 input
    
let part2 input = 
    let draw pixel sprite = if [sprite-1;sprite;sprite+1] |> Seq.exists (fun s -> s = pixel ) then '#' else '.'
    
    let rec part2' cycle x screen = function
        | head::tail -> let pixel = draw (cycle % 40) x
                        part2' (cycle + 1) (execute x head) (pixel::screen) tail
        | [] -> screen |> Seq.rev
        
    part2' 0 1 [] input
    
let input = File.ReadAllLines("input") |> Seq.map parse |> Seq.collect id |> Seq.toList
input |> part1 |> printfn "%i" 
input |> part2 |> Seq.chunkBySize 40 |> Seq.map System.String |> Seq.iter (printfn "%s")