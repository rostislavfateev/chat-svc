# Chat Service


## General Design Considerations

### Components:
- App - singleton tying things together
    - Container with rooms, managing their lifecycle
    - Container with users, 
    - Container for general configuration values/bounds
    - Server for connections and their promotion
    - Handles registration of new users
- Room - implements chat functionality
    - Broadcasts messages from users
    - Holds conversation history
- Peer - connection representation
    - Rx Tx channel for message exchange
    - History of sent messages
    - Inactivity timer (might be built-in)

### Requests

- 

