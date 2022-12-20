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
let visited = HashSet<string>()

let rec calculateMaxPressure location minutes (allValves: Map<string,int>) interestingValves connections =
    let maxPressure' (location, minutes, valves) =
        let baseScore = allValves[location] * minutes
        let bestValve, score =
            valves
            |> Seq.map (fun v -> v, (bfs connections (location, v)) + 1)
            |> Seq.filter ( fun (_,pathLength) -> pathLength <= minutes)
            |> Seq.map (fun (v, pathLength) -> v, calculateMaxPressure v (minutes - pathLength) allValves (Set.remove v valves) connections)
            |> (fun s -> if Seq.isEmpty s then "",0 else Seq.maxBy snd s)
        visited.Add(bestValve) |> ignore
        baseScore + score
        
        
    let key = (location, minutes, interestingValves)
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

calculateMaxPressure "AA" 30 allValves interestingValves connections |> printfn "%A"

// Part 2 doesn't work. Elephant should be run in parallel to human, because human has time to open all interesting valves,
//  so elephant does not open any valves with this approach
visited.Clear()
let human = calculateMaxPressure "AA" 26 allValves interestingValves connections
let nonVisitedValves = Seq.except visited interestingValves |> Set.ofSeq
let elephant = calculateMaxPressure "AA" 26 allValves nonVisitedValves connections
printfn "%A" (human + elephant)
