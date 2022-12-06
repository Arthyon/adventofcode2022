open System.Collections.Generic
open System.IO

let rec findMarker lookahead pos (input: 'a list) =
    if HashSet(input[..lookahead - 1]).Count = lookahead then pos + lookahead else findMarker lookahead (pos + 1) input[1..]
    
let input = File.ReadAllText("input").ToCharArray() |> Seq.toList
input |> findMarker 4 0 |> printfn "%A"
input |> findMarker 14 0 |> printfn "%A"


