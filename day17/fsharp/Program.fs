open System.Collections.Generic
open System.IO


let shapes = [
    [ (0, 0); (1, 0); (2, 0); (3, 0) ] // -
    [ (1, 2); (0, 1); (1, 1); (2, 1); (1, 0) ] // +
    [ (2, 2); (2, 1);(0, 0); (1, 0); (2, 0) ] // J
    [ (0, 3); (0, 2); (0, 1); (0, 0) ] // I
    [ (0, 1); (1, 1); (0, 0); (1, 0) ]  // .
]

let input = File.ReadAllText("input").ToCharArray() |> Seq.indexed
let movement = seq { while true do yield! input }

let spawn height shape = shape |> List.map (fun (x,y) -> x + 2, y + height + 3)
        
        
let hasRoomToMove rockPos bottom =
        Seq.forall2 (fun rock bottom -> rock < 0 || rock > bottom) rockPos bottom
                
let jetstream rock movement =
        rock |> List.map (fun (x,y) -> match movement with 
                                       | '>' -> x + 1,y
                                       | '<' -> x - 1,y
                                       | x -> failwithf "fail %A" x)
                
        
let canMoveRock rock tower =
        let outOfBounds (x,y) = x < 0 || x >= 7 || y < 0
        rock |> Seq.exists (fun point -> Set.contains point tower || outOfBounds point) |> not
        
        
let rec fall tower rock (movement: IEnumerator<int * char>) =
        movement.MoveNext() |> ignore
        let idx,nextMovement = movement.Current
        
        let nextRock = jetstream rock nextMovement
        let rock = if canMoveRock nextRock tower then nextRock else rock
        let fallenRock = rock |> List.map (fun (x,y) -> x, y - 1)
        match canMoveRock fallenRock tower with
        | false ->
                (Set.union (Set.ofList rock) tower), idx
        | true  ->
                fall tower fallenRock movement
        
let rockfall tower shape movement =
        let height = if Set.isEmpty tower then 0 else tower |> Seq.map snd |> Seq.max |> (+) 1
        let rock = spawn height shape
        fall tower rock movement
        
        
let movementEnumerator = movement.GetEnumerator()

let mutable tower = Set.empty
for i = 0 to 2021 do
      let nextShape = shapes[i % 5]
      let newTower, _ = rockfall tower nextShape movementEnumerator
      tower <- newTower
      
        
tower |> Seq.map snd |> Seq.max |> (+) 1 |> printfn "%A"

// Part 2
let mutable tower2 = Set.empty
let mutable rocks = 0UL
let mutable rockShape = -1
let movementEnumerator2 = movement.GetEnumerator()
let mutable seen = Map.empty
let mutable addedByRepeating = 0UL
let mutable target = 1000000000000UL

while rocks <= target do
      rockShape <- (rockShape + 1) % 5
      let nextShape = shapes[rockShape]
      let newTower, idx = rockfall tower2 nextShape movementEnumerator2
      tower2 <- newTower
      
      let key = rockShape,idx
      let top = tower2 |> Seq.map snd |> Seq.max
      let repeats,oldPieces,oldTop = if seen.ContainsKey(key) then seen[key] else 0,0UL,0
      if addedByRepeating = 0UL then
              if repeats = 3 then
                      let deltaTop = top - oldTop
                      let deltaPieces = rocks - oldPieces
                      let repeats = (target - rocks) / uint64 deltaPieces
                      addedByRepeating <- repeats * (uint64 deltaTop)
                      rocks <- rocks + (repeats * (uint64 deltaPieces))
                      
              seen <- Map.add key (repeats + 1, rocks, top) seen
              
      rocks <- rocks + 1UL
        
tower2 |> Seq.map snd |> Seq.map uint64 |> Seq.max |> (+) addedByRepeating |> printfn "%A"