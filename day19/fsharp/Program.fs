open System.Collections.Generic
open System.IO
open System.Text.RegularExpressions

let ore,clay,obsidian,geode = 0,1,2,3

type Robot = {
    robotType: int
    cost: int list
}

type Blueprint = {
    number: int
    oreRobot: Robot
    clayRobot: Robot
    obsidianRobot: Robot
    geodeRobot: Robot
    costMatrix: int[,]
}

type State = {
    resources: int list
    robots: int list
}

let parse line =
    let m = Regex("Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.")
                .Match(line)
    let v (i: int) = m.Groups[i].Value |> int
    {
      number = v 1
      oreRobot = { robotType = ore ; cost = [v 2;0;0;0] }
      clayRobot = { robotType = clay ; cost = [v 3;0;0;0] }
      obsidianRobot = { robotType = obsidian ; cost = [v 4;v 5;0;0] }
      geodeRobot = { robotType = geode ; cost = [v 6;0;v 7;0] }
      costMatrix = [[v 2; v 3; v 4; v 6;]; [0;0; v 5; 0] ;[0;0;0;v 7] ;[0;0;0;0]] |> array2D
    }
    
let calculateQualityLevel (blueprint,geodes) = geodes * blueprint.number

let canAfford robot state = Seq.forall2 (fun res cost -> res >= cost) state.resources robot.cost

let needMoreRobots blueprint state productionType =
    let costs = blueprint.costMatrix[productionType, *]
    state.robots[productionType] < (costs |> Seq.max)
    
let buyRobot robot state =
    let resources = List.map2 (fun res cost -> res - cost) state.resources robot.cost
    let robots = state.robots |> List.mapi (fun i r -> if i = robot.robotType then r + 1 else r)
    { resources = resources ; robots = robots }
    
let incrementResources state =
    { state with resources = state.resources |> List.mapi (fun i r -> r + state.robots[i]) }

let determineNextStates blueprint minute state = seq {
        if minute = 1 then
            yield (incrementResources state)
        else    
            if canAfford blueprint.geodeRobot state then
                yield (state |> incrementResources |> buyRobot blueprint.geodeRobot)
            else 
                yield (incrementResources state)
                
                if needMoreRobots blueprint state obsidian && canAfford blueprint.obsidianRobot state then
                    yield (state |> incrementResources |> buyRobot blueprint.obsidianRobot)
                else
                    if needMoreRobots blueprint state clay && canAfford blueprint.clayRobot state then
                        yield (state |> incrementResources |> buyRobot blueprint.clayRobot)
                    if needMoreRobots blueprint state ore && canAfford blueprint.oreRobot state then
                        yield (state |> incrementResources |> buyRobot blueprint.oreRobot)
                
    }
    
let cache = Dictionary<_,_>()
let cacheKey minute state blueprint =
    let s = state.resources |> Seq.append state.robots |> Seq.map string |> String.concat ""
    $"{minute}{s}{blueprint.number}"
    
let rec evaluateBluePrint minute state blueprint =
    if minute = 0 then
            blueprint, state.resources[geode]
    else
        determineNextStates blueprint minute state
                        |> Seq.map (evaluateBranch minute blueprint)
                        |> Seq.max
and evaluateBranch minute blueprint state =
    let key = cacheKey minute state blueprint
    let success, value = cache.TryGetValue(key)
    if success
    then value
    else
        let value = evaluateBluePrint (minute - 1) state blueprint
        cache.Add(key, value)
        value
        
let input = File.ReadAllLines("input") |> Seq.map parse |> Seq.toList

let initialState = {resources = [0;0;0;0]; robots = [1;0;0;0]}

input |> Seq.map (evaluateBluePrint 24 initialState) |> Seq.map calculateQualityLevel |> Seq.sum |> printfn "%A"
// Works on input, not on sample. I got lucky with the pruning heuristics
input |> Seq.truncate 3 |> Seq.map (evaluateBluePrint 32 initialState) |> Seq.map snd |> Seq.reduce (*) |> printfn "%A"
