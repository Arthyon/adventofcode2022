open System.Collections.Generic
open System.IO

type Point = int * int
type MountainMap = {
    // blizzards: Set<Point * Point> // position, direction
    height: int
    width: int
    start: Point
    ending: Point
}

let modulo x m = (x%m + m)%m

let parseDirection y x el =
    match el with
    | '<' -> Some((x,y), (-1, 0))
    | '^' -> Some((x,y), (0, -1))
    | '>' -> Some((x,y), (1,0))
    | 'v' -> Some((x,y), (0,1))
    | _ -> None

let isEmptySpot c = c = '.'
let rec mapBlizzards map =
    map |> Array2D.mapi parseDirection |> Seq.cast<Option<Point * Point>> |> Seq.choose id |> Set.ofSeq
    
let parseMap lines =
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
    [(x,y)
     (x, y + 1)
     (x, y - 1)
     (x + 1, y )
     (x - 1, y )] |> Seq.filter canMove |> Seq.toList
    
let getPossibleNodes pos map blizzards =
    getNeighbours pos map |> Seq.filter (fun pos -> blizzards |> Set.contains pos |> not)
        

let nextItem (q : Queue<int * int * int>) =
    match q.Count with
    | 0 -> None
    | _ -> Some (q.Dequeue())


let walk mountain (maps: Map<int, Set<Point>>) (startX,startY) dest startMinute =

    let updateQueueAndVisited t (q : Queue<int * int * int>, v) (x, y) =
        q.Enqueue(x, y, (t + 1))
        q, v |> Set.add (x, y, (t + 1) % maps.Count)
        
    let rec loop (queue : Queue<int * int * int>, visited: Set<int * int * int>) =
        match nextItem queue with
        | Some (x, y, time) ->
            // visited.Add (x,y, time % maps.Count)
            let nextMinute = time + 1
            let mapIdx = nextMinute % maps.Count
            let map = maps |> Map.find(mapIdx)
            let nodes = getPossibleNodes (x,y) mountain map
            if nodes |> Seq.contains dest then
                time + 1
            else
                nodes |> Seq.map (fun (x,y) -> x,y, mapIdx)
                |> Seq.filter (visited.Contains >> not)
                |> Seq.map (fun (x,y,_) -> x,y)
                |> Seq.fold (updateQueueAndVisited time) (queue,visited)
                |> loop
                // |> Seq.iter (fun (x,y,_) -> queue.Enqueue(x,y, nextMinute))
                // loop (queue, visited |> Set.add (x,y,time % maps.Count))
                
                // |> Seq.fold (updateQueueAndVisited time) (queue, visited)
            // let nl = (r, c) |> next // next position
            // if nl |> Seq.contains (dr, dc) then // to destination
            //     time + 1 // return time
            // else
            //     nl
            //     |> Seq.filter (notVisiitedAndEmpty visited t)
            //     |> Seq.fold (updateQueueAndVisited t) (queue, visited)
            //     |> loop
        | _ -> -1

    let q = Queue [ (startX,startY, startMinute) ]

    // queue.Enqueue(sr, sc, baseTime)
    loop (q, Set.empty)
    
let bfs mountain (maps: Map<int, Set<Point>>) (x,y) dest startMinute =
    let visited = HashSet [ x,y, startMinute ]
    let q = Queue [ (x,y, startMinute) ]

    let rec visit () =
        match q.TryDequeue() with
        | false, _ -> failwith "todo"
        | true, (x,y, minute) ->
            if (x,y) = dest then
                minute
            else
                visited.Add (x,y, minute % maps.Count) |> ignore
                let nextMinute = minute + 1
                let mapIdx = nextMinute % maps.Count
                let map = maps |> Map.find(mapIdx)
                let nodes = getPossibleNodes (x,y) mountain map
                nodes |> Seq.map (fun (x,y) -> x,y, mapIdx)
                |> Seq.filter (visited.Contains >> not)
                |> Seq.iter (fun (x,y,_) -> q.Enqueue(x,y, nextMinute))

                visit ()

    visit ()
    
    
let rec findCycle width height maps =
        let original = maps |> Seq.last
        let next = maps |> Seq.head |> moveBlizzards width height
        
        if original = next then
            maps |> Seq.map (fun s -> s |> Set.map fst) |> Seq.rev |> Seq.indexed |> Map.ofSeq
        else
            findCycle width height (next::maps)
        
        
    
let map, blizzards = File.ReadAllLines("input") |> parseMap
let maps = [blizzards] |> findCycle map.width map.height

let part1 = walk map maps map.start map.ending 0
printfn "%A" part1
let back = walk map maps map.ending map.start part1
let part2 = walk map maps map.start map.ending back
printfn "part2: %A" part2