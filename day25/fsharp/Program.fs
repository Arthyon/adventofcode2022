open System.IO
open fsharp.Snafu

File.ReadAllLines("input") |> Seq.map snafu |> Seq.reduce (+) |> printfn "%A"