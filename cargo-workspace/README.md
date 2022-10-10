# Cargo Workspace
Allows build hierarchy of projects for better maintenance and shorter compile times

* client - client project, uses shared lib
* server - server project, uses shared lib
* shared - API shared by client and server


### Build all
```cargo build```

### Run server
```cargo run --bin server```

### Run client
``` cargo run --bin client```
