open System.IO
open Microsoft.FSharp.Collections

let toCoords x y = function | '#' -> Some(x,y) | _ -> None

type Direction = | N | S | E | W | NE | NW | SE | SW
let getCoordinates (x,y) = function
    | N -> (x - 1, y)
    | S -> (x + 1, y)
    | E -> (x, y + 1)
    | W -> (x, y - 1)
    | NE -> (x - 1, y + 1)
    | NW -> (x - 1, y - 1)
    | SE -> (x + 1, y + 1)
    | SW -> (x + 1, y - 1)
    
let vacant pos others directions =
    directions |> Seq.map (getCoordinates pos) |> Seq.filter (fun c -> others |> Set.contains c) |> Seq.isEmpty

let proposeMove others (directions: Direction list list) start pos =
    let isVacant = vacant pos others
    if isVacant (directions |> Seq.collect id) then None
    else if isVacant directions[start] then Some(pos, getCoordinates pos directions[start].[0])
    else if isVacant directions[(start + 1) % 4] then Some(pos, getCoordinates pos directions[(start + 1) % 4].[0])
    else if isVacant directions[(start + 2) % 4] then Some(pos, getCoordinates pos directions[(start + 2) % 4].[0])
    else if isVacant directions[(start + 3) % 4] then Some(pos, getCoordinates pos directions[(start + 3) % 4].[0])
    else None
    
let proposeMoves start =
    let mutable start = start
    let directions = [
        [N;NE;NW]
        [S;SE;SW]
        [W;NW;SW]
        [E;NE;SE]
    ]
    let proposeMoves' positions =
        let moves = positions |> Seq.choose (proposeMove positions directions start)
        start <- (start + 1) % 4
        moves
            |> Seq.groupBy snd
            |> Seq.filter (fun (_, proposals) -> (proposals |> Seq.length) = 1)
            |> Seq.collect snd
            |> Seq.toList
        
    proposeMoves' 
    
let moveElves positions validMoves =
    let replace pos =
        let replacement = validMoves |> Seq.tryFind (fun (o, _) -> o = pos)
        if replacement.IsSome then replacement.Value |> snd else pos
    positions |> Set.map replace
    
let getBounds positions = 
    let minX = positions |> Set.map fst |> Set.minElement
    let maxX = positions |> Set.map fst |> Set.maxElement
    let minY = positions |> Set.map snd |> Set.minElement
    let maxY = positions |> Set.map snd |> Set.maxElement
    minX, maxX, minY, maxY
    
let rec performRounds remainingRounds moveProposer positions =
    if remainingRounds = 0
    then positions
    else
        positions |> moveProposer |> moveElves positions |> performRounds (remainingRounds - 1) moveProposer
    
let rec moveUntilNotNeeded rounds moveProposer positions =
    let moves = positions |> moveProposer |> Seq.toList
    
    if moves.Length = 0
    then rounds
    else moveElves positions moves |> moveUntilNotNeeded (rounds + 1) moveProposer
                
    
let calculateEmptyTiles positions =
    let minX, maxX, minY, maxY = getBounds positions
    let area = (maxX - minX + 1) * (maxY - minY + 1)
    area - positions.Count
    
let input = File.ReadAllLines("input")
            |> Seq.map (fun l -> l.ToCharArray())
            |> array2D
            |> Array2D.mapi toCoords
            |> Seq.cast<Option<int * int>>
            |> Seq.choose id
            |> Set.ofSeq

performRounds 10 (proposeMoves 0) input |> calculateEmptyTiles |> printfn "%A"
moveUntilNotNeeded 1 (proposeMoves 0) input |> printfn "%A"
