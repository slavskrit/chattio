# Example of rust streeam with gRPC and reflections


```bash
grpc_cli call localhost:50051 chat.ChatService.GetMessages  "name:'123'" 
```

## Task

### Create a chat client

Given next proto structure:

https://github.com/slavskrit/chattio/blob/a9ce123b5531287609560d4c9a339600171fcf07/proto/chat.proto#L1-L27


Steps:
- Take proto structure
- Implement frontend functionality for the service
- Assuming auth is any provided token
- Implement Chat

Chat:
- Chat is a stream of messages
- Messages can have different type, refer to proto
- Chat is in read-only mode, so no bother with that
- Messages can be sent by different people, so this should be implemented as well

