open System
open System.IO

let getPriority (c: char) = if Char.IsLower c then int c - 96 else int c - 38

let splitCompartments (s: string) =
    let left, right = s |> Seq.toList |> List.splitAt (s.Length / 2)
    seq {left |> Seq.toArray |> String ;right |> Seq.toArray |> String}

let intersect (groups: string seq) =
    seq {
        for s in groups do
            Set.ofArray (s.ToCharArray())
    }
    |> Set.intersectMany
    |> Seq.exactlyOne

let getItemPriority = intersect >> getPriority
let input = File.ReadAllLines("input")
    
// part 1
input
    |> Seq.map splitCompartments
    |> Seq.map getItemPriority
    |> Seq.sum
    |> printfn "%A"
    
// part 2
input
    |> Seq.chunkBySize 3
    |> Seq.map getItemPriority
    |> Seq.sum
    |> printfn "%A"
