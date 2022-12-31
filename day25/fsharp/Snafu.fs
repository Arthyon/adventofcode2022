module fsharp.Snafu

open System

let private toNumber = function | '2' -> 2m | '1' -> 1m | '0' -> 0m | '-' -> -1m | '=' -> -2m | _ -> failwith "Unknown snafu digit"

let private getDecimals (i: float) = i - Math.Truncate i


let private toSnafu (i: float) =
    let rec toSnafu' (i:float) part snafu =
        if part < 0 then snafu |> Seq.rev |> Seq.toArray |> String
        else
            let divisor = pown 5.0  part
            let value = i / divisor
            let newVal,snafu =
                if value >= 1.5 then
                    i - divisor * 2.0,'2'::snafu
                else if value >= 1 then
                    i - divisor,'1'::snafu
                else if getDecimals value >= 0.5 then
                    i - divisor,'1'::snafu
                else if value <= -1.5 then
                    i + divisor * 2.0, '='::snafu
                else if value <= -0.5 then
                    i + divisor, '-'::snafu
                else i,'0'::snafu
            toSnafu' newVal (part - 1) snafu
            
    let exp = Math.Log(i, 5)
    let decimals = getDecimals exp
    let part = int <| if decimals > 0.5 then Math.Ceiling exp else Math.Floor exp
    
    toSnafu' i part []
    
[<StructuredFormatDisplay("{Value}")>]
type Snafu = private { underlyingValue: decimal }
                member m.Value = m.ToString()
                override this.ToString() =
                    this.underlyingValue |> float |> toSnafu
                static member (+) (n1: Snafu, n2 : Snafu) =
                    { underlyingValue = n1.underlyingValue + n2.underlyingValue }
                    
let snafu s = { underlyingValue = s |> Seq.map toNumber |> Seq.rev |> Seq.toArray |> Seq.indexed |> Seq.map (fun (idx,el) -> el * (pown 5m idx)) |> Seq.reduce (+) }

