open System.Collections.Generic
open System.IO

type Point = int * int
type MountainMap = {
    height: int
    width: int
    start: Point
    ending: Point
}

let parseDirection y x el =
    match el with
    | '<' -> Some((x,y), (-1, 0))
    | '^' -> Some((x,y), (0, -1))
    | '>' -> Some((x,y), (1,0))
    | 'v' -> Some((x,y), (0,1))
    | _ -> None

let rec mapBlizzards map =
    map |> Array2D.mapi parseDirection |> Seq.cast<Option<Point * Point>> |> Seq.choose id |> Set.ofSeq
    
let parseMap lines =
    let isEmptySpot c = c = '.'
    let array = lines |> Seq.map (fun l -> l |> Seq.cast<char>) |> array2D
    let start = array[0,*] |> Seq.findIndex isEmptySpot
    let ending = array[^0,*] |> Seq.findIndex isEmptySpot
    {
        height = (Array2D.length1 array) - 1
        width = (Array2D.length2 array) - 1
        start = start,0
        ending = ending, Array2D.length1 array - 1
    }, mapBlizzards array
    
let wrap length = function
    | 0 -> length - 1
    | p when p = length -> 1
    | p -> p
    
let moveBlizzard width height blizzard =
    let (x,y), (xDir, yDir) = blizzard
    let x = x + xDir |> wrap width
    let y = y + yDir |> wrap height
    (x,y),(xDir,yDir)
    
let moveBlizzards width height blizzards =    
    blizzards |> Set.map (moveBlizzard width height)

let getNeighbours (x,y) map =
    let canMove (x,y) =
        let isEnding = (x,y) = map.ending
        let isStart = (x,y) = map.start
        let isNotWall = x > 0 && y > 0 && x < map.width && y < map.height
        isEnding || isStart || isNotWall
    [(x,y); (x, y + 1); (x, y - 1); (x + 1, y ); (x - 1, y )] |> Seq.filter canMove
    
let getPossibleNodes pos map blizzards =
    getNeighbours pos map |> Seq.filter (fun pos -> blizzards |> Set.contains pos |> not) |> Seq.toList
        
let nextItem (q : Queue<int * int * int>) =
    match q.Count with
    | 0 -> None
    | _ -> Some (q.Dequeue())

let bfs mountain (maps: Map<int, Set<Point>>) (startX,startY) dest startMinute =

    let enqueueAndVisit time (queue : Queue<int * int * int>, visited) (x, y) =
        queue.Enqueue(x, y, time)
        queue, visited |> Set.add (x, y, time % maps.Count)
        
    let isNotVisited time visited (x,y) =
        visited |> Set.contains (x,y,time) |> not
        
    let rec loop (queue : Queue<int * int * int>, visited: Set<int * int * int>) =
        match nextItem queue with
        | Some (x, y, time) ->
            let nextTime = time + 1
            let nextMap = nextTime % maps.Count
            let nodes = maps |> Map.find(nextMap) |> getPossibleNodes (x,y) mountain
            if nodes |> Seq.contains dest then
                nextTime
            else
                nodes
                |> Seq.filter (isNotVisited nextMap visited)
                |> Seq.fold (enqueueAndVisit nextTime) (queue, visited)
                |> loop
        | _ -> failwith "Couldn't find path"

    let queue = Queue [ (startX,startY, startMinute) ]

    loop (queue, Set.empty)
    
let rec findCycle width height maps =
        let original = maps |> Seq.last
        let next = maps |> Seq.head |> moveBlizzards width height
        
        if original = next then
            maps |> Seq.map (fun s -> s |> Set.map fst) |> Seq.rev |> Seq.indexed |> Map.ofSeq
        else
            findCycle width height (next::maps)
        
let map, blizzards = File.ReadAllLines("input") |> parseMap
let maps = [blizzards] |> findCycle map.width map.height

let part1 = bfs map maps map.start map.ending 0
printfn "%A" part1

let back = bfs map maps map.ending map.start part1
let part2 = bfs map maps map.start map.ending back
printfn "%A" part2