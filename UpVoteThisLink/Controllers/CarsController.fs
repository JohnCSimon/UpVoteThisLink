namespace UpVoteThisLink.Controllers
open System
open System.Collections.Generic
open System.Linq
open System.Net.Http
open System.Web.Http
open UpVoteThisLink.Models

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

type CarsController() =
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
        let (upvoteRecord, metadata) = Query.all<UpVoteRecord> |> Query.where <@ fun g s -> s.PartitionKey = shortKey @> |> fromUpVoteTable |> Seq.exactlyOne
        let modifiedGame = { upvoteRecord with UpVotes = upvoteRecord.UpVotes + 1}
        let result = (modifiedGame, metadata.Etag) |> Replace |> inUpVoteTable 
        (modifiedGame, metadata.Etag)   

    /// Gets all values.
    member x.Get() = "you get nothing!"

    // get specific one
    
    member x.Get(key: string) = getUpVotesForShortKey key

    // upvote!
    member x.Post(key: string) = doUpVoteByShortkey key 
    
    member x.Put(key: string) = doUpVoteByShortkey key 