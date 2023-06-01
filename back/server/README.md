# Server

## Goal

- Listen to any mint event on the blockchain, and link a `token_id` to an `image_id`
  - Done in `src/listener.rs`
- Have a web server to be able to get the attributes of an image and th image itself from a `token_id`
  - Done in `src/image.rs`
  - Routes:
    - GET `/data/{token_id}`: Get the metadata of the image
    - GET `/image/{token_id}`: Get the image itself
    - GET `/data`: Debug route to get all the metadata

## Run

### Environment variables

- `WSS_URL`: The websocket url to listen to

### Init database

```bash
./resources/db/setup.sh
```

This will create a `app.db` file (sqlite3) in the `resources` folder.

### Load images

When all images are generated, copy the content of output folder of `generator/resources/output` into `server/resources/images`.