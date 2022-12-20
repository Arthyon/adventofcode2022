open System.Collections.Generic
open System.IO

let modulo x m = (x%m + m)%m
let createList (ints: ('a * 'b) list) indices =
    seq { for i in indices do yield (snd ints[i]) }

let mix (input: (int * int64) list) count =
    let indices = input |> List.map fst |> List
    for i = 1 to count do
        for idx,value in input do
            let currIdx = indices.IndexOf(idx)
            indices.RemoveAt(currIdx)
            let newIndex = modulo (int64 currIdx + value) (int64 indices.Count)
            indices.Insert(int newIndex, idx)
    createList input indices |> Seq.toList
    
let decode (list: int64 list) =
    let startIndex = list |> Seq.findIndex (fun v -> v = 0L) |> int64
    let length = int64 list.Length
    [1000L;2000L;3000L]
        |> Seq.map (fun n -> modulo (startIndex + n) length)
        |> Seq.map (fun idx -> list[int idx])
        |> Seq.sum
    
let input = File.ReadAllLines("input") |> Seq.map int64 |> Seq.indexed |> Seq.toList

mix input 1 |> decode |> printfn "%A"

let part2input = input |> Seq.map ( fun (i,value) -> i, value * 811589153L) |> Seq.toList
mix part2input 10 |> decode |> printfn "%A"

    
 
    