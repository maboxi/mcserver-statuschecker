# mcserver-statuschecker: A REST API for querying minecraft servers with a basic web interface

## REST API:

Endpoint ```/api```:
- ```/api/servers```: List all monitored servers
- ```/api/servers/<server id>```: Query information about a specific server

Endpoint ```/api/servers/<server id>```
- ```./status```: Query server status information
- ```./icon```: Query the server's icon

## Configuration

Example configuration:

```JSON
{
    "servers": [
        {
            "name": "Local",
            "id": "local",
            "host": "localhost"
        },
        {
            "name": "My server",
            "id": "myserver",
            "host": "mc.myserver.com",
            "port": 25566
        }
    ]
}
```