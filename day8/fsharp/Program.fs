open System
open System.IO

let input = File.ReadAllLines("input")
            |> Seq.map (Seq.map Char.GetNumericValue)
            |> array2D

let checkDirectionForVisibility el direction =
    direction |> Seq.filter (fun t -> t >= el) |> Seq.isEmpty
    
let isVisible (north, south, east, west) el =
    let visible = checkDirectionForVisibility el
    visible north || visible south || visible east || visible west
    
let getViewDistance el direction =
    let distance = (direction |> Seq.takeWhile (fun x -> x < el) |> Seq.length)
    if distance = (direction |> Seq.length) then distance else distance + 1
    
let calculateScenicScore (north, south, east, west) el =
    let viewDistance = getViewDistance el
    viewDistance (north |> Seq.rev) * viewDistance south * viewDistance east * viewDistance (west |> Seq.rev)
    
let mapForest (array: float[,]) xCount yCount mapper x y el =
    let north = array[0..x, y][..^1]
    let west = array[x, 0..y][..^1]
    let east = array[x, y..xCount][1..]
    let south = array[x..yCount, y][1..]
    mapper (north, south, east, west) el
    
let part1 = mapForest input (Array2D.length1 input) (Array2D.length2 input) isVisible
let part2 = mapForest input (Array2D.length1 input) (Array2D.length2 input) calculateScenicScore
input |> Array2D.mapi part1 |> Seq.cast |> Seq.filter id |> Seq.length |> printfn "%i"
input |> Array2D.mapi part2 |> Seq.cast |> Seq.max |> printfn "%i"
