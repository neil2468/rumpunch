# Spec


## DDos Protection

- Rate limiting

## Privacy

Peer must designate a whitelist of `PeerId`s allowed to know its information. 


How to restrict that only original peer can set its whitelist?
- Create a IP:Port and PeerId associataion? First come, first served.


Alice and Bob want to hole-punch to each other. Alice and Bob only know each
other's `PeerId`. They must use a server Sid. We don't want a rogue peer Rob
to interrupt the operations of Alice or Bob, or to learn about Alice or Bob's
private data. 


## PeerId

- A UTF-8 string
- Should be length limited to allow knowing maximum size of receive UDP packets


## Messages

- Max size should be within typical MTU values and allow for header data
    - Maybe 508 bytes. 
    See https://stackoverflow.com/questions/1098897/what-is-the-largest-safe-udp-packet-size-on-the-internet


**Ack**: Server acknowledges reception of a message

**Query**: A peer can query the server for their characterisation.

**Test**: A peer sends a message to the server which, in sum, allow the server 
to charactrise the current behaviour of the peer's NAT and determine which
hole-punching method should be used to connect to another peer.





# Deleteme 

Alice and Bob want to connect. They only know each other's PeerId and the public
internet address of a Server.

Alice tells the Server its own PeerId and that they they want to connect to Bob. 
Server records the request and the current public internet address of Alice.
Server will respond with whether the matching Start message has been received
from the other peer. The peer should retry the Start message until it receives
a 

[Message `Start Alice Bob`]

Bob tells the Server its own PeerID and that they they want to connect to Alice.
Server records the request and the current public internet address of Bob.


Alice sends multiple messsages to Server to allow the Server to
characterise Alice's NAT's current behaviour.

[Message `Sample Alice`]

Bob send multiple messages to Server to allow the Server to characterise
Bob's NAT's current behaviour. 

Alice asks Server for hole-punching method and values to connect to Bob. Server
may not be ready to respond and will let Alice know. Eventually the Server will
respond or Alice will give up and, maybe, start again. 

[Message `QueryResult Alice Bob`]

Bob asks Server for hole-punching method and values to connect to Alice. Server
may not be ready to respond and will let Bob know. Eventually the Server will
respond or Bob will give up and, maybe, start again. 


- Server will use rate limiting, based on IP address (not port) for all messages.
Max 20 messages in 1 minute allowed. 

- Server will ignore Sample and QueryResult messages if insufficient Start mesages were received.

- Should server expect a peer to always use the same IP address (NO!) or should
they expect them to use only IP addresses 'near' one another.
    - How to determine IP address distance?
    











