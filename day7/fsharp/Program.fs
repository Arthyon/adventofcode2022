open System.Collections.Generic

type FileInfo = { name: string ; size: int }
type DirectoryInfo = { name: string ; parent: Option<DirectoryInfo> ; children: List<Item> }
and Item =
    | File of FileInfo
    | Directory of DirectoryInfo
let rec traverseToRoot dir = match dir.parent with | Some(parent) -> traverseToRoot parent | None -> dir
let isDir = function | Directory d -> Some d | _ -> None
let getChildDirectory dir name = dir.children |> Seq.choose isDir |> Seq.find (fun x -> x.name = name)
let (|Cd|Ls|Dir|File|EOF|) (s:string) =
    if System.String.IsNullOrEmpty s then EOF()
    else if s.StartsWith("$ cd") then Cd(s.Substring(5))
    else if s.StartsWith("$ ls") then Ls()
    else if s.StartsWith("dir") then Dir(s.Substring(4))
    else
        let l = s.Split(' ')
        File(int l[0], l[1])
        
let rec parse (reader: System.IO.StreamReader) (currentDir: DirectoryInfo) =
    let line = reader.ReadLine()
    match line with
    | Cd dir when dir = "/" ->
        traverseToRoot currentDir |> parse reader
    | Cd dir when dir = ".." ->
        parse reader currentDir.parent.Value
    | Cd dir ->
        getChildDirectory currentDir dir |> parse reader
    | Ls     -> parse reader currentDir
    | Dir name ->
        currentDir.children.Add(Directory({name = name; parent = Some(currentDir); children = List<Item>()}))
        parse reader currentDir
    | File (size, name) ->
        currentDir.children.Add(File({name = name;size = size}))
        parse reader currentDir
    | EOF -> Directory(traverseToRoot currentDir)
    
let getDirectorySizes tree =
    let rec accumulateDirectories (directories: List<int>) acc = function
        | Item.File fileInfo -> acc + fileInfo.size
        | Directory directoryInfo ->
             let sizeOfChildren = directoryInfo.children |> Seq.fold (accumulateDirectories directories) 0
             
             directories.Add(sizeOfChildren)
             sizeOfChildren + acc
             
    let directories = List<int>()
    let totalSize = accumulateDirectories directories 0 tree
    directories, totalSize
                
let reader = new System.IO.StreamReader("input")
let root = parse reader {name = "/" ; parent = None; children = List<Item>()}
let directories, totalSize = getDirectorySizes root

// Part 1
directories |> Seq.filter (fun x -> x <= 100000) |> Seq.sum |> printfn "%i"
// Part 2
let freeSpace = 70000000 - totalSize
let neededSpace = 30000000 - freeSpace
directories |> Seq.filter (fun x -> x >= neededSpace) |> Seq.min |> printfn "%i"
