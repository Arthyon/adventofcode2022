open System
open System.IO
open System.Text.RegularExpressions

type Sensor = {
    coords: int * int
    beacon: int * int
    length: int
}

let getPathLength (x1:int,y1:int) (x2,y2) = Math.Abs(x1 - x2) + Math.Abs(y1 - y2)

let parse line =
    let rx = Regex("Sensor at x=(\d+), y=(\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
    let getter = rx.Match line |> (fun m (idx:int) -> int m.Groups[idx].Value)
    let x1,y1,x2,y2 = getter 1, getter 2, getter 3, getter 4
    let length = getPathLength (x1,y1) (x2,y2)
    { coords = x1,y1 ; beacon = x2,y2 ; length = length }
    
let getCoverage xMin xMax y sensor =
    seq {
        for x = xMin to xMax do
            let length = getPathLength sensor.coords (x,y)
            if sensor.beacon <> (x,y) && length <= sensor.length then (x,y)
    } |> Seq.toArray
        
let toSpectrum x sensor =
    let sx,sy = sensor.coords
    let distance = sensor.length - Math.Abs(x - sx)
    if distance < 0
    then
        None
    else
        Some(x, (sy - distance), (sy + distance))
    
let analyzeSpectrum (max, _) (x, minY, maxY) =
    if minY <= max + 1 then
        Math.Max(maxY, max), None
    else
        max, Some(x, max + 1)
    
let analyzeRow sensors x =
    let m = sensors |> Seq.choose (toSpectrum x) |> Seq.sortBy (fun (_,minY,_) -> minY) |> Seq.scan analyzeSpectrum (0, None) |> Seq.skipWhile (fun (_,r) -> r.IsNone) |> Seq.tryHead
    match m with
    | None -> None
    | Some (_, None) -> None
    | Some (_, Some(x,y)) -> Some((uint64 x, uint64 y))
    
let calculateTuningFrequency (x,y) =
    x * 4000000UL + y
    
let sensors = File.ReadLines("input") |> Seq.map parse |> Seq.toArray

// Part 1
let beacons = sensors |> Seq.map (fun s -> s.beacon)
let xCoords = sensors |> Seq.map (fun s -> [fst s.coords + s.length ; fst s.coords - s.length]) |> Seq.collect id

let xMin = xCoords |> Seq.min
let xMax = xCoords |> Seq.max

let part1 = Array.Parallel.collect (getCoverage xMin xMax 2000000) >> Seq.distinct >> Seq.length
sensors |> part1 |> printfn "%A"
 
// Part 2
let maxY = 4000000
    
[0..maxY]
    |> Seq.pick (analyzeRow sensors)
    |> calculateTuningFrequency
    |> printfn "%A"