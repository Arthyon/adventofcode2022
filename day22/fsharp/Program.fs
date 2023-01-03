open System.IO
open System.Text.RegularExpressions
open fsharp.Cube

let modulo x m = (x%m + m)%m

type Block = | Walkable | Wall | Edge
type Movement = | Walk of int | TurnLeft | TurnRight
type Player = { pos: int * int ; dir: char }

let left,up,down,right = '<','^','v','>'
let directions = [right;down;left;up]

let parseMovement (line: string) = seq {
        for m in Regex("(\d+)([RL]?)").Matches(line) do
            yield Walk(m.Groups[1].Value |> int)
            
            let dir = m.Groups[2].Value
            if dir = "R" then yield TurnRight else if dir = "L" then yield TurnLeft
    } 
        
let parseLine line = line |> Seq.map (fun c -> if c = '.' then Walkable else if c = '#' then Wall else if c = ' ' then Edge else failwith "Invalid") |> Seq.toList
let parseMap lines =
    lines |> Seq.map parseLine |> Seq.toList
    
let rec move (path: Block list) pos dir lastWalkablePos steps =
    if steps = 0 then pos
    else
        let newPos = modulo (pos + dir) path.Length
        let nextBlock = path[newPos]
        match nextBlock with
        | Edge -> move path newPos dir lastWalkablePos steps 
        | Wall when path[pos] = Edge -> lastWalkablePos
        | Wall -> pos
        | Walkable -> move path newPos dir newPos (steps - 1)
        
let moveToNewPos (map: Block list list) player steps =
    let dir = if player.dir = up || player.dir = left then -1 else 1
    let pos, path =
        match player.dir with
        | '^' | 'v' -> let m = map |> Seq.transpose |> Seq.map Seq.toList |> Seq.toList
                       fst player.pos,m[snd player.pos] |> Seq.toList
        | '>' | '<' -> snd player.pos, map[fst player.pos]
        | _ -> failwith "fail"
            
    let newPos = move path pos dir pos steps
    if player.dir = right || player.dir = left
    then { player with pos = fst player.pos, newPos}
    else { player with pos = newPos, snd player.pos }
    
let addTups (x1,y1) (x2,y2) = x1 + x2, y1 + y2

let getMovementDirection dir =
    match dir with
    | '^' -> (-1,0)
    | 'v' -> (1,0)
    | '<' -> (0,-1)
    | '>' -> (0,1)
    | _ -> failwith "todo"
    
let getNextBlock (map: Block list list) (x,y) =
    if x >= 0 && map.Length > x && y >= 0 && map[x].Length > y then map[x][y] else Edge
    
//let rec teleportMove map teleport player steps =
//    let pos,dir = teleport player.pos player.dir
//    let nextBlock = getNextBlock map pos
//    match nextBlock with
//    | Wall -> player,steps, Wall
//    | Walkable ->
//        {player with pos = pos; dir = dir},steps, Walkable
//    | Edge -> failwithf "todo2"
//    // | Edge -> teleportMove map teleport { player with pos = pos; dir = dir } (steps - 1)
    
let rec cubeMove (map: Block list list) player movement steps teleport =
    if steps = 0 then player
    else
        let newPos = addTups player.pos movement
        let nextBlock = getNextBlock map newPos
        match nextBlock with
        | Edge ->
            
            let pos,dir = teleport player.pos player.dir
            let nextBlock = getNextBlock map pos
            match nextBlock with
            | Wall -> player
            | Walkable ->
                let newMovement = getMovementDirection dir
                cubeMove map {player with pos = pos; dir = dir} newMovement (steps - 1) teleport
            | Edge -> failwith "todo2"
        | Wall -> player
        | Walkable -> cubeMove map {player with pos = newPos} movement (steps - 1) teleport
        
    
let moveToNewPosCube teleport (map: Block list list) player steps =
    let dir = getMovementDirection player.dir
            
    cubeMove map player dir steps teleport
        
let movePlayer map player moveImpl = function
    | Walk steps -> moveImpl map player steps
    | TurnLeft   -> let index = directions |> Seq.findIndex (fun c -> c = player.dir)
                    { player with dir = directions[modulo (index - 1) 4]}
    | TurnRight  -> let index = directions |> Seq.findIndex (fun c -> c = player.dir)
                    { player with dir = directions[modulo (index + 1) 4]}

let rec followPath map player moveImpl = function
    | movement::rest -> let newPlayer = movePlayer map player moveImpl movement
                        followPath map newPlayer moveImpl rest
    | [] -> player
    
let printPassword player =
    let row = (fst player.pos + 1) * 1000
    let col = (snd player.pos + 1) * 4
    let dir = directions |> Seq.findIndex (fun c -> c = player.dir)
    [row;col;dir] |> Seq.sum |> printfn "%A"
    
        
let fileName = "input"
let input = File.ReadAllLines(fileName)
let movement = parseMovement input[^0] |> Seq.toList
let map = parseMap input[..^2]

// part 1
let player = { pos = 0, map[0] |> Seq.findIndex ( fun c -> match c with | Walkable -> true | _ -> false) ; dir = right }
followPath map player moveToNewPos movement |> printPassword


// part 2
let player2 = { pos = 0, map[0] |> Seq.findIndex ( fun c -> match c with | Walkable -> true | _ -> false) ; dir = right }

let faces = if fileName = "input" then getCurrentRealFaces else getCurrentTestFaces
let teleport = teleport faces
followPath map player2 (moveToNewPosCube teleport) movement |> printPassword