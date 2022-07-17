# Development documentation

Documentation regarding the development process of this project, including how to set up the project locally.

### Requirements

All requirements to run the application locally.

```
1. Rust
2. MongoDB
3. Optional: docker and docker-compose
```

### Local installation

All steps to install and run the application locally.

```
1. Clone the repo in your desired folder
2. Create a MongoDB collection, which equals the environment APP_NAME variable
3. Fill in the environment variables (.env file) where needed
4. Run the development command `cargo watch -x run` (this automatically installs the dependencies)
```

Test the local environment by going to the specified url with the path `/api/status/ping`. If successful, a json message `ping` will be returned.

**Notice:** This setup has no data stored in the local MongoDB collection. Send a love letter to one of the fellow developers to retrieve a database export.  

### Environment variables

Copy and paste the following into the project root `.env` file. Change the variables where needed.

```
# rocket variables
ROCKET_port=8000
ROCKET_ident="whereto"

# app variables
APP_NAME="databin"

# database variables
mongo_url="localhost"
mongo_port="27017"
```
