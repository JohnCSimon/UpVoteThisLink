module TableOperationsModule

open Microsoft.WindowsAzure.Storage
open Microsoft.WindowsAzure.Storage.Table
open DigitallyCreated.FSharp.Azure.TableStorage

open Microsoft.WindowsAzure.Storage
open Microsoft.WindowsAzure.Storage.Table

let account = CloudStorageAccount.Parse "UseDevelopmentStorage=true;" //Or your connection string here
let tableClient = account.CreateCloudTableClient()