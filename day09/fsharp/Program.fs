open System
open System.Collections.Generic
open System.IO
open System.Linq

let follow (xHead, yHead) (xTail, yTail) =
    let xdiff:int = xHead - xTail
    let yDiff:int = yHead - yTail
    match Math.Abs(xdiff), Math.Abs(yDiff) with
    | 0,0 | 0,1 | 1,0 | 1,1 -> xTail, yTail
    | 1,_ -> (xHead, yTail + Math.Sign(yDiff))
    | _,1 -> (xTail + Math.Sign(xdiff), yHead)
    | _,_ -> (xTail + Math.Sign(xdiff), yTail + Math.Sign(yDiff))
       
let move (x, y) = function
    | "R" -> (x + 1, y)
    | "L" -> (x - 1, y)
    | "U" -> (x, y + 1)
    | "D" -> (x, y - 1)
    | _ -> failwith "Invalid direction"
    
let parseLine (line: string)=
    let parts = line.Split(" ")
    Enumerable.Repeat(parts[0], int parts[1])
        
let moveRope length directions =    
    let rope = Enumerable.Repeat((0,0), length) |> Seq.toArray
    let positions = HashSet<int * int>()
    positions.Add(rope |> Seq.last) |> ignore
    for direction in directions do
        rope[0] <- move rope[0] direction
        for i = 1 to rope.Length - 1 do
            rope[i] <- follow rope[i - 1] rope[i]
        positions.Add(rope |> Seq.last) |> ignore
    positions.Count
    
let input = File.ReadAllLines("input") |> Seq.collect parseLine

input |> moveRope 2 |> printfn "%A"
input |> moveRope 10 |> printfn "%A"

    