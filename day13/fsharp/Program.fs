open System.IO

type Item =
    | Number of int
    | List of Item list

let parseNumber c (tail: char list) =
    let idx = tail |> List.tryFindIndex(fun x -> x = ',' || x = ']')
    let numberParts, tail = if idx.IsSome then tail |> List.splitAt idx.Value else [],[]
    let number = c::numberParts |> Seq.toArray |> System.String// |> int

    Number(int number), tail

let extractArray chars =
    let mutable indents = 0
    let mutable arr = []
    let mutable rest = []
    let mutable found = false
    for c in chars do
        if found
        then
            rest <- c::rest
        else
            arr <- c::arr
            match c with
            | '[' -> indents <- indents + 1
            | ']' when indents > 0 -> indents <- indents - 1
            | ']' when indents = 0 -> found <- true
            | _ -> ()
    '['::(arr |> Seq.rev |> Seq.toList),(rest |> Seq.rev |> Seq.toList)

let rec parseItem items = function
    | '['::tail -> let arr, h = extractArray tail

                   let l = arr |> Seq.toArray |> System.String |> parseArray
                   parseItem (l::items) h

    | ','::tail -> parseItem items tail
    | x::tail -> let n, t = parseNumber x tail
                 parseItem (n::items) t
    | [] -> items |> Seq.rev |> Seq.toList

and parseArray (line: string) =
    let items = line[1..^1] |> Seq.toList |> parseItem []
    List(items)

            
    
let rec compare (first: Item) (second: Item) =
    match (first, second) with
    | Number x, Number y -> x.CompareTo(y)
    | Number _, List _ -> compare (List [first]) second
    | List _, Number _ -> compare first (List [second])
    | List x, List y ->
        let xLength, yLength = (List.length x),(List.length y)
        let smallerLength = min xLength yLength
        List.zip (List.take smallerLength x) (List.take smallerLength y)
        |> List.map (fun (x, y) -> compare x y)
        |> List.skipWhile (fun c -> c = 0)
        |> List.tryHead
        |> function
            | Some c -> c
            | None -> xLength.CompareTo(yLength)

    
let isInOrder i (el: Item seq) =
    let first = el |> Seq.head
    let second = el |> Seq.last
    if (compare first second) < 0 then i + 1 else 0
    
let input = File.ReadLines("input")
            |> Seq.filter (System.String.IsNullOrWhiteSpace >> not)
            |> Seq.map parseArray
            
input
    |> Seq.chunkBySize 2
    |> Seq.mapi isInOrder
    |> Seq.sum
    |> printfn "%i"


let divider1 = parseArray "[[2]]"
let divider2 = parseArray "[[6]]"

input
    |> Seq.toArray
    |> Array.append [| divider1; divider2 |]
    |> Array.sortWith compare
    |> Array.indexed
    |> Array.filter (fun (_, c) -> c = divider1 || c = divider2)
    |> Array.map fst
    |> Array.reduce (*)
    |> printfn "%A"

    
    
