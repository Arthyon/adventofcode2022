open System.Collections.Generic
open System.IO

let parse (line:string) =
    let parts = line.Split(',')
    int parts[0], int parts[1], int parts[2]
    
let minmax set fn =
    set |> Seq.map fn |> Seq.max, set |> Seq.map fn |> Seq.min
    
let getBoundingBox cubes =
    let maxX, minX = minmax cubes (fun (x,_,_) -> x)
    let maxY, minY = minmax cubes (fun (_,y,_) -> y)
    let maxZ, minZ = minmax cubes (fun (_,_,z) -> z)
    (maxX + 1, maxY + 1, maxZ + 1), (minX - 1, minY - 1, minZ - 1)
    
let neighbours (x,y,z) =
    seq {
        yield x, y + 1, z
        yield x + 1, y, z
        yield x, y, z + 1
        yield x, y - 1, z
        yield x - 1, y, z
        yield x, y, z - 1
    }
    
let nextItem (q : Queue<int * int * int>) =
    match q.Count with
    | 0 -> None
    | _ -> Some (q.Dequeue())

let inBounds bounds (x,y,z) =
     let (maxX, maxY, maxZ), (minX, minY, minZ) = bounds
     let isX = x <= maxX && x >= minX
     let isY = y <= maxY && y >= minY
     let isZ = z <= maxZ && z >= minZ
     isX && isY && isZ
    
let fill bounds cubes =
    let surface = Set.empty
    let _,min = bounds
    let air = Set[min]
    let q = Queue [min]
    let rec fill' surface air queue =
        match nextItem queue with
        | Some(pos) ->
            let neighbours = neighbours pos |> Seq.toList |> Seq.filter (inBounds bounds) |> Seq.filter (fun pos -> (Set.contains pos air) |> not) |> Seq.toList
            let surface = neighbours |> Seq.filter (fun pos -> Set.contains pos cubes) |> Set.ofSeq |> Set.union surface
            let validNeighbours = neighbours |> Seq.except surface |> Set.ofSeq
            
            validNeighbours |> Seq.iter queue.Enqueue
            fill' surface (Set.union air validNeighbours) queue
            
        | None -> surface |> Seq.collect neighbours |> Seq.filter (fun x -> (Set.contains x air))
        
    fill' surface air q
    
    
    
let cubes = File.ReadAllLines("input") |> Seq.map parse |> Set.ofSeq
let box = getBoundingBox cubes

cubes |> Seq.collect neighbours |> Seq.filter (fun x -> (Set.contains x cubes) |> not) |> Seq.length |> printfn "%A"

cubes |> fill box |> Seq.length |> printfn "%A"


    