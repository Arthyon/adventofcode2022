open System.Collections.Generic
open System.IO

type Status = | AtRest | HitTarget

let getCoordinates (part1: string, part2: string) =    
    let left = part1.Split(',') |> Seq.map int |> Seq.toList
    let right = part2.Split(',') |> Seq.map int |> Seq.toList
    ([int left[0]; int right[0]] |> Seq.sort |> Seq.toList, [int left[1]; int right[1]] |> Seq.sort |> Seq.toList) 
    
let parse (line:string) =
    let coords = line.Split("->") |> Seq.pairwise |> Seq.map getCoordinates
    seq {
        for x,y in coords do
            yield! if x[0] = x[1]
                   then [y[0]..y[1]] |> Seq.map (fun y -> x[0],y)
                   else [x[0]..x[1]] |> Seq.map (fun x -> x,y[0])
    }
    
let rec fall1 (bottom:int) (items: HashSet<int * int>) = function
    | _,y when y = bottom -> HitTarget
    | x,y when  items.Contains(x, y + 1) |> not -> fall1 bottom items (x,y+1)
    | x,y when items.Contains(x - 1, y + 1) |> not -> fall1 bottom items (x - 1,y+1)
    | x,y when items.Contains(x + 1, y + 1) |> not -> fall1 bottom items (x + 1,y+1)
    | coords -> items.Add(coords) |> ignore
                AtRest
      
let rec fall2 (bottom:int) (items: HashSet<int * int>) = function
    | x,y when y + 1 = bottom -> items.Add(x,y) |> ignore
                                 AtRest
    | x,y when  items.Contains(x, y + 1) |> not -> fall2 bottom items (x,y+1)
    | x,y when items.Contains(x - 1, y + 1) |> not -> fall2 bottom items (x - 1,y+1)
    | x,y when items.Contains(x + 1, y + 1) |> not -> fall2 bottom items (x + 1,y+1)
    | 500,0 -> HitTarget
    | coords -> items.Add(coords) |> ignore
                AtRest
         
let addSand fall =
    let rec addSand' no fall =
        let start = (500,0)
        match fall start with 
        | AtRest -> addSand' (no + 1) fall
        | HitTarget -> no
    addSand' 0 fall
    
let input = File.ReadAllLines("input") |> Seq.map parse |> Seq.collect id |> Seq.toList

let _, yMax = input |> Seq.maxBy snd
let part1 = fall1 yMax (HashSet(input))
addSand part1 |> printfn "%A"

let part2 = fall2 (yMax + 2) (HashSet(input))
addSand part2 |> (fun i -> i + 1) |> printfn "%A"

