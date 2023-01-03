module fsharp.Cube

let left,up,down,right = '<','^','v','>'

type Position = int * int
type FaceData = { name: string ; top: Position; size: int ; right: Position * char * bool ; left: Position * char * bool; up: Position * char * bool; down:Position * char * bool }

let getFaceDelta (deltaX, deltaY) size inverted = function
    | '<','<' -> deltaX, size
    | '>','>' when inverted -> size - deltaX, size
    | '>','>' -> deltaX, 0
    | '^', '^' when inverted -> size, deltaY
    | '^', '^' -> size, deltaY
    | 'v', 'v' -> 0, deltaY
    | '<', '^' -> size, size - deltaX
    | '<', 'v' -> 0, deltaX
    | '>', 'v' -> 0, size - deltaX
    | '>', '<' -> size - deltaX,size
    | '^', '>' -> deltaY,0
    | '^', '<' -> size - deltaY, size
    | '^', 'v' -> 0, size - deltaY
    | 'v', '>' -> size - deltaY, 0
    | 'v', '^' -> size, size - deltaY
    | '>', '<' -> size - deltaX,size
    | '<', '>' -> size - deltaX,0
    | '>', '^' -> size,deltaX
    | 'v', '<' -> deltaY, size
    | dirs -> failwithf "%A not implemented" dirs
    
let nextFace face = function
    | '>' -> face.right
    | '^' -> face.up
    | '<' -> face.left
    | 'v' -> face.down
    | _ -> failwith "invalid dir"

let getCurrentFace faces (x,y) =
    faces |> Seq.find (fun f ->
        let left,top = f.top
        let right = left + f.size
        let bottom = top + f.size
        x >= left && x <= right && y >= top && y <= bottom
    )
    
let Tone,Ttwo,Tthree,Tfour,Tfive,Tsix = (0,8),(4,0),(4,4),(4,8),(8,8),(8,12)
let testSize = 3 // one less than face size
let testFaces = [
    {name = "one" ; top = Tone ; size = testSize ; right = Tsix,'v',false;left = Tthree,'v',false;up = Ttwo,'v',false;down = Tfour,'v',false} // 1
    {name = "two" ;top = Ttwo ;size = testSize ; right = Tthree,'>',false;left = Tsix,'<',false;up = Tone,'v',false;down = Tfive,'^',false} // 2
    {name = "three" ;top = Tthree ;size = testSize ; right = Tfour,'>',false;left = Ttwo,'<',false;up = Tone,'>',false;down = Tfive,'>',false} // 3
    {name = "four" ;top = Tfour ;size = testSize ; right = Tsix,'v',false;left = Tthree,'<',false;up = Tone,'^',false;down = Tfive,'v',false} // 4
    {name = "five" ;top = Tfive ; size = testSize ;right = Tsix,'>',false;left = Tthree,'^',false;up = Tfour,'^',false;down = Ttwo,'^',false} // 5
    {name = "six" ; top = Tsix ; size = testSize ;right = Tone,'<',false;left = Tfive,'<',false;up = Tfour,'<',false;down = Ttwo,'>',false} // 6
]
let getCurrentTestFaces = getCurrentFace testFaces

let one,two,three,four,five,six = (0,50),(0,100),(50,50),(100,0),(100,50),(150,0)
let realSize = 49 // one less than face size
let realFaces = [
    {name = "one" ; top = one ; size = realSize ; right = two,'>',false;left = four,'>',false;up = six,'>',false;down = three,'v',false} // 1
    {name = "two" ;top = two ;size = realSize ; right = five,'<',true;left = one,'<',false;up = six,'^',true;down = three,'<',false} // 2 right and up folding has changed! opposite
    {name = "three" ;top = three ;size = realSize ; right = two,'^',false;left = four,'v',false;up = one,'^',false;down = five,'v',false} // 3
    {name = "four" ;top = four ;size = realSize ; right = five,'>',false;left = one,'>',false;up = three,'>',false;down = six,'v',false} // 4
    {name = "five" ;top = five ; size = realSize ;right = two,'<',false;left = four,'<',false;up = three,'^',false;down = six,'<',false} // 5
    {name = "six" ; top = six ; size = realSize ;right = five,'^',false;left = one,'v',false;up = four,'^',false;down = two,'v',false} // 6
]
let getCurrentRealFaces = getCurrentFace realFaces

let teleport getFaces (x,y) dir =
    let face = getFaces (x,y)
    
    let nextPos,nextDir,inverted = nextFace face dir
    let xDelta, yDelta = getFaceDelta (x - fst face.top, y - snd face.top) face.size inverted (dir, nextDir)
    let newPos = (fst nextPos) + xDelta, (snd nextPos) + yDelta
    newPos,nextDir