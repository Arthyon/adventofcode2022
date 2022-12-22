open System.IO
open System.Text.RegularExpressions

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
    
// let printBlock = function
//     | Walkable -> printf "."
//     | Edge -> printf " "
//     | Wall -> printf "#"
//     
// let printMap (map: 'a list list) player =
//     for x = 0 to (map.Length - 1) do
//         for y = 0 to (map[x].Length - 1) do
//             if player.pos = (x,y) then
//                 printf $"{player.dir}"
//             else printBlock (map[x][y])
//         printfn ""
//     printfn ""
    
let rec move (path: Block List) pos dir lastWalkablePos steps =
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
        
let movePlayer map player = function
    | Walk steps -> moveToNewPos map player steps
    | TurnLeft   -> let index = directions |> Seq.findIndex (fun c -> c = player.dir)
                    { player with dir = directions[modulo (index - 1) 4]}
    | TurnRight  -> let index = directions |> Seq.findIndex (fun c -> c = player.dir)
                    { player with dir = directions[modulo (index + 1) 4]}

let rec followPath map player = function
    | movement::rest -> let newPlayer = movePlayer map player movement
                        followPath map newPlayer rest
    | [] -> player
        
let input = File.ReadAllLines("input")
let movement = parseMovement input[^0] |> Seq.toList
let map = parseMap input[..^2]

let player = { pos = 0, map[0] |> Seq.findIndex ( fun c -> match c with | Walkable -> true | _ -> false) ; dir = right }
let ending = followPath map player movement

let row = (fst ending.pos + 1) * 1000
let col = (snd ending.pos + 1) * 4
let dir = directions |> Seq.findIndex (fun c -> c = ending.dir)
[row;col;dir] |> Seq.sum |> printfn "%A"

    