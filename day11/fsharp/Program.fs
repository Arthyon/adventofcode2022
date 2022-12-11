open System.Collections.Generic
open System.IO
open System.Text.RegularExpressions
open Microsoft.FSharp.Linq.RuntimeHelpers

type Monkey = {
    id: int
    items: List<uint64>
    operation: uint64 -> uint64
    throwTo: uint64 -> int
    divisor: int
    mutable itemsInspected: uint64
}

let operator = function
    | "+" -> <@ (+) @>
    | "-" -> <@ (-) @>
    | "*" -> <@ (*) @>
    | "/" -> <@ (/) @>
    | _ -> failwith "unsupported operator"
    
let exp s = if s = "old" then <@ id @> else <@ fun _ -> uint64 s @>
    
let createOperation (left:string) o (right:string) : uint64 -> uint64 =
    <@ fun old -> (%operator o) ((%exp left) old) ((%exp right) old) @> |> LeafExpressionConverter.EvaluateQuotation |> unbox

let rec parse lines =
    let rx = Regex("Monkey (\d+):\n\
                   \s{2}Starting items: (.*)\n\
                   \s{2}Operation: new = (.*) ([+-/\*]) (.*)\n\
                   \s{2}Test: divisible by (\d+)\n\
                   \s{4}If true: throw to monkey (\d+)\n\
                   \s{4}If false: throw to monkey (\d+)")
    lines |> String.concat "\n" |> rx.Match |> (fun m ->
    {
        id = int m.Groups[1].Value
        items = m.Groups[2].Value.Split(',') |> Seq.map uint64 |> List<uint64>
        operation = createOperation m.Groups[3].Value m.Groups[4].Value m.Groups[5].Value
        divisor = (int m.Groups[6].Value)
        throwTo = fun x -> if x % (uint64 m.Groups[6].Value) = 0UL then int m.Groups[7].Value else int m.Groups[8].Value
        itemsInspected = 0UL
    })
    
let parseInput input = input |> Seq.chunkBySize 7 |> Seq.map parse |> Seq.toList

let getMonkeyBusiness monkeys = monkeys |> Seq.map (fun x -> x.itemsInspected) |> Seq.sortDescending |> Seq.take 2 |> Seq.toList |> (fun m -> m[1] * m[^1])

let performRound worryReducer (monkeys: Monkey list) =
    for monkey in monkeys do
        for item in monkey.items do
            let worry = worryReducer (monkey.operation item)
            let newOwner = monkey.throwTo worry
            monkeys[newOwner].items.Add(worry)
            monkey.itemsInspected <- monkey.itemsInspected + 1UL
        monkey.items.Clear()

let input = File.ReadAllLines("input")

let part1Monkeys = parseInput input
let part1WorryReducer worry = worry / 3UL
for i in 1..20 do
     performRound part1WorryReducer part1Monkeys
     
getMonkeyBusiness part1Monkeys |> printfn "%i"
    
let part2Monkeys = parseInput input
let divisionConstant = part2Monkeys |> Seq.map (fun m -> m.divisor) |> Seq.fold ( fun a b -> a * b) 1 |> uint64
let part2WorryReducer worry = worry % divisionConstant
for i in 1..10000 do
     performRound part2WorryReducer part2Monkeys
     
getMonkeyBusiness part2Monkeys |> printfn "%i"
