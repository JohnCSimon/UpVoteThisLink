namespace UpVoteThisLink.Controllers
open System
open System.Collections.Generic
open System.Linq
open System.Net.Http
open System.Web.Http

open Microsoft.WindowsAzure.Storage
open Microsoft.WindowsAzure.Storage.Table
open DigitallyCreated.FSharp.Azure.TableStorage

open Microsoft.WindowsAzure.Storage
open Microsoft.WindowsAzure.Storage.Table


type UpVoteRecord = 
    { [<PartitionKey>] PartitionKey: string
      [<RowKey>] RowKey: string
      Url: string 
      UpVotes: int
      }

type UVTLController() = 
    inherit ApiController()

    let account = CloudStorageAccount.Parse "UseDevelopmentStorage=true;" 
    let tableClient = account.CreateCloudTableClient()
    let inUpVoteTable q = inTable tableClient "UpVoteThisLinkStorage" q
    let fromUpVoteTable q = fromTable tableClient "UpVoteThisLinkStorage" q

    let getUpVotesForShortKey shortKey = 
            Query.all<UpVoteRecord>
            |> Query.where <@ fun g s -> s.PartitionKey = shortKey @>
            |> fromUpVoteTable

    let doUpVoteByShortkey shortKey =
        let queryResult = Query.all<UpVoteRecord> |> Query.where <@ fun g s -> s.PartitionKey = shortKey @> |> fromUpVoteTable 
        if queryResult |> Seq.length = 1 then
            let (upvoteRecord, metadata) = queryResult |> Seq.exactlyOne
            let modifiedGame = { upvoteRecord with UpVotes = upvoteRecord.UpVotes + 1}
            let result = (modifiedGame, metadata.Etag) |> Replace |> inUpVoteTable
            modifiedGame
        else 
            {PartitionKey = ""; RowKey = ""; Url = ""; UpVotes = 0 }

    let insertByUrl url username = 
        let uvr = {PartitionKey = string(url.GetHashCode()); RowKey = username; Url = url; UpVotes = 0 }
        let result = uvr |> Insert |> inUpVoteTable
        result

    member x.Get() = "you get nothing!"
    
    member x.Get(key: string) = getUpVotesForShortKey key

    member x.Post(key: string) = doUpVoteByShortkey key 
    
    member x.Put(url: string, username) = insertByUrl url username

