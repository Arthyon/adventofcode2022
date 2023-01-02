open System.Collections.Generic
open System.IO
open System.Text.RegularExpressions

type Tunnel = {
    name: string
    rate: int
    connections: Set<string>
}

let parse line =
    let m = Regex("Valve (.*) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.*)").Match(line)
    { name = m.Groups[1].Value ; rate = m.Groups[2].Value |> int ; connections = m.Groups[3].Value.Split(",") |> Seq.map (fun i -> i.Trim()) |> Set.ofSeq }

let getConnections tunnel = tunnel.name, tunnel.connections
let getValve tunnel = tunnel.name, tunnel.rate

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
            
let bfs (connections: Map<string, Set<string>>) =
    let bfs' (src, dest) =
        let visited = HashSet [ src ]
        let q = Queue [ (src, 0) ]

        let rec visit () =
            match q.TryDequeue() with
            | false, _ -> failwith "todo"
            | true, (location, distance) ->
                if location = dest then
                    distance
                else
                    visited.Add location |> ignore

                    connections[location]
                    |> Set.filter (visited.Contains >> not)
                    |> Seq.iter (fun next -> q.Enqueue(next, distance + 1))

                    visit ()

        visit ()

    memoize bfs'
    
let cache = Dictionary<_,_>()

let rec calculateMaxPressure location minutes (allValves: Map<string,int>) interestingValves connections elephant =
    let maxPressure' (location, minutes, valves, elephant) =
        let baseScore = allValves[location] * minutes
        let score =
            valves
            |> Seq.map (fun v -> v, (bfs connections (location, v)) + 1)
            |> Seq.filter ( fun (_,pathLength) -> pathLength <= minutes)
            |> Seq.map (fun (v, pathLength) -> calculateMaxPressure v (minutes - pathLength) allValves (Set.remove v valves) connections elephant)
            |> (fun s -> if Seq.isEmpty s then 0 else Seq.max s)
            |> (fun score ->
                if elephant then
                        let human = calculateMaxPressure "AA" 26 allValves valves connections false
                        max score human
                else
                    score)
            
        baseScore + score
        
        
    let key = (location, minutes, interestingValves, elephant)
    match cache.TryGetValue key with
    | true, value -> value
    | false, _    ->
        let value = maxPressure' key
        cache.Add(key, value)
        value
    
    
let input = File.ReadAllLines("input") |> Seq.map parse |> Seq.toList

let connections = input |> Seq.map getConnections |> Map.ofSeq
let allValves = input |> Seq.map getValve |> Map.ofSeq
let interestingValves = allValves |> Map.filter (fun _ v -> v > 0) |> Map.keys |> Set.ofSeq

calculateMaxPressure "AA" 30 allValves interestingValves connections false |> printfn "%A"

calculateMaxPressure "AA" 26 allValves interestingValves connections true |> printfn "%A"
