open System.IO
open System.Text.RegularExpressions

let rx = Regex(@"(\d+)\-(\d+),(\d+)\-(\d+)", RegexOptions.Compiled)
    
let extractRanges line =
    let m = rx.Match line
    if m.Success |> not then failwith "Wrong structure"
    m.Groups.Values |> Seq.skip 1 |> Seq.map (fun g -> int g.Value) |> Seq.toList
    
let prepareRanges (ranges: int list) =
    [ranges[0], ranges[1];ranges[2], ranges[3]]
        |> Seq.sortBy (fun (x,y) -> x,-y)
        |> Seq.toList
    
let isFullOverlap (ranges: (int * int) list) = snd ranges[0] >= snd ranges[1]
let isPartialOverlap (ranges: (int * int) list) = isFullOverlap ranges || snd ranges[0] >= fst ranges[1]
    
let input = File.ReadAllLines("input")
input |> Seq.map extractRanges |> Seq.map prepareRanges |> Seq.filter isFullOverlap |> Seq.length |> printfn "%A"
input |> Seq.map extractRanges |> Seq.map prepareRanges |> Seq.filter isPartialOverlap |> Seq.length |> printfn "%A"

