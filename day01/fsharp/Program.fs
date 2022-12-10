open System.IO

let (|Int|_|) (str:string) =
    match System.Int32.TryParse str with
    | true,int -> Some int
    | _ -> None

let calculate reader =
    let rec calculate' current totals (reader: StreamReader) =
        match reader.ReadLine() with
        | null  -> current::totals
        | ""    -> calculate' 0 (current::totals) reader
        | Int i -> calculate' (current + i) totals reader
        | s     -> failwith $"{s} is not an integer"
    calculate' 0 [] reader
    
let totalElves = 3
let calories = new StreamReader("input.txt")
               |> calculate 
               |> Seq.sortDescending
               |> Seq.take totalElves
               |> Seq.sum
               
printf $"Total calories for {totalElves} elves: {calories}"
