open System
open System.Collections.Generic
open System.IO

type Node = { 
    Coords: int*int
    mutable Parent: Option<Node>
    mutable Cost: int
    height: int
}        

let getHeight = function | 'S' -> 0 | 'E' -> 25 | x   -> int x - 97
let findPositions char arr =
    arr |> Array2D.mapi (fun x y el -> if el = char then Some(x,y) else None) |> Seq.cast<Option<int * int>> |> Seq.choose id |> Seq.toArray
    
let input = File.ReadAllLines("input") |> Seq.map Seq.toList |> array2D
let width, height = (Array2D.length1 input), (Array2D.length2 input)
let heights = input |> Array2D.map getHeight
    
let getNeighbours node =
    let generateCoords (x,y) = seq { (x+1, y); (x, y+1); (x-1, y); (x, y-1) } |> Seq.filter (fun (x,y) -> x >= 0 && x < width && y >= 0 && y < height)
    let isClimbable destination = node.height + 1 >= heights[fst destination, snd destination]
    
    generateCoords node.Coords
        |> Seq.filter isClimbable
        |> Seq.map (fun (x,y) -> { Coords = x,y; Parent = Some(node); Cost = node.Cost + 1; height = heights[x,y]})
    
let existsIn list node = list |> Seq.exists(fun x -> x.Coords = node.Coords)

let findCost startNode endNode =
    let openSet = List<Node>()
    let closedSet = List<Node>()
    openSet.Add(startNode)
    
    while not(endNode |> existsIn closedSet) do
        let currentNode = openSet |> Seq.minBy (fun n -> n.Cost)
        openSet.Remove(currentNode) |> ignore
        closedSet.Add(currentNode)
        let neighbours = getNeighbours currentNode |> Seq.filter ((existsIn closedSet) >> not)
        for node in neighbours do
            match openSet |> Seq.tryFind (fun n -> n.Coords = node.Coords) with
            | None -> openSet.Add(node)
                      node.Parent <- Some(currentNode)
                      node.Cost <- currentNode.Cost + 1
            | Some(n) -> let newCost = currentNode.Cost + 1
                         if newCost < n.Cost then
                             n.Parent <- Some(currentNode)
                             n.Cost <- newCost
        
        if openSet.Count = 0 then closedSet.Add(endNode) // No path to goal. break out of loop
        
    let endNode = closedSet.Find(fun n -> n.Coords = endNode.Coords)
    if endNode.Parent.IsSome then endNode.Cost else 9999

let findCostFromPosition pos =
    let startNode = { Coords = pos; Cost = 0; height = heights[fst pos, snd pos]; Parent = None }
    let goal = findPositions 'E' input |> Seq.head
    let endNode = { Coords = goal; Cost = Int32.MaxValue; height = heights[fst goal, snd goal]; Parent = None }
    findCost startNode endNode
    
    
// Part 1
findPositions 'S' input |> Seq.head |> findCostFromPosition |> printfn "%A" 

// Part 2
findPositions 'a' input |> Array.Parallel.map findCostFromPosition |> Seq.min |> printfn "%A"
    
