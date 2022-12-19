open System.Collections.Generic
open System.IO


let shapes = [
    [ (0, 0); (1, 0); (2, 0); (3, 0) ] // -
    [ (1, 2); (0, 1); (1, 1); (2, 1); (1, 0) ] // +
    [ (2, 2); (2, 1);(0, 0); (1, 0); (2, 0) ] // J
    [ (0, 3); (0, 2); (0, 1); (0, 0) ] // I
    [ (0, 1); (1, 1); (0, 0); (1, 0) ]  // .
]

let input = File.ReadAllText("input").ToCharArray()
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
        
        
let rec fall tower rock (movement: IEnumerator<char>) =
        movement.MoveNext() |> ignore
        let nextMovement = movement.Current
        
        let nextRock = jetstream rock nextMovement
        let rock = if canMoveRock nextRock tower then nextRock else rock
        let fallenRock = rock |> List.map (fun (x,y) -> x, y - 1)
        match canMoveRock fallenRock tower with
        | false ->
                Set.union (Set.ofList rock) tower
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
      tower <- rockfall tower nextShape movementEnumerator
      
        
tower |> Seq.map snd |> Seq.max |> (+) 1 |> printfn "%A"

// 3142 too high