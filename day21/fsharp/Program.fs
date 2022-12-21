open System.Collections.Generic
open System.IO
open System.Text.RegularExpressions

type Yell =
    | Number of int64
    | Operation of string * string * string
    
let parse (input: string) =
    let parts = input.Split(':')
    let m = Regex("(.*) ([+-/*]) (.*)").Match(parts[1].Trim())
    if m.Success
    then
        parts[0], Operation(m.Groups[1].Value, m.Groups[2].Value, m.Groups[3].Value)
    else
        parts[0], Number(int64 parts[1])


let memoize f =
    let dict = Dictionary<_,_>()
    fun c ->
        let exist, value = dict.TryGetValue c
        match exist with
        | true -> value
        | false ->
            let value = f c
            dict.Add(c, value)
            value
            
let input = File.ReadAllLines("input") |> Seq.map parse |> Map.ofSeq

let calculate operator (left,right) =
    match operator with
    | "+" -> left + right
    | "-" -> left - right
    | "/" -> left / right
    | "*" -> left * right
    | _ -> failwith "Unknown operator"
            
let rec find (monkeys: Map<string, Yell>) (monkey: string) =
    match monkeys.TryGetValue(monkey) with
    | false, _ -> failwithf $"Invalid monkey %s{monkey}"
    | true, Number i -> i
    | true, Operation(left, op, right) -> [left;right] |> Seq.map memFind |> Seq.pairwise |> Seq.head |> calculate op
        
and memFind = memoize find input

let reduceFormula (l:string) =
    if l.Contains("x") then l
    else
        let s = l.Trim('(').Trim(')').Split(' ')
        if s.Length = 1 then s[0]
        else
        
            let l = int64 s[0]
            let r = int64 s[2]
            calculate s[1] (l,r) |> string

let rec findFormula (monkeys: Map<string, Yell>) (monkey: string) =
    match monkeys.TryGetValue(monkey) with
    | false, _ -> failwithf $"Invalid monkey %s{monkey}"
    | true, Number _ when monkey = "humn" -> "x"
    | true, Number i -> $"%i{i}"
    | true, Operation(left, op, right) -> [left;right]
                                          |> Seq.map memFindFormula
                                          |> Seq.map reduceFormula
                                          |> Seq.pairwise
                                          |> Seq.head
                                          |> (fun (l,r) -> $"({l} {op} {r})")
                                          |> reduceFormula
        
and memFindFormula = memoize findFormula input

// Part 1        
memFind "root"  |> printfn "%A"

// Part 2
let (Operation(left, _, right)) = input["root"]
let leftFormula = memFindFormula left
let rightFormula = memFindFormula right
if leftFormula.Contains("x") then
    let rightVal = memFind right
    printfn "%A = %A" leftFormula rightVal
else 
    let leftVal = memFind left
    printfn "%A = %A" rightFormula leftVal
    
// Then paste the formula into an online math solver