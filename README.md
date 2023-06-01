# 

## Image Generator

_Not the best code, but as it's goal is to be run only once, it's ok._

### How to use

- In resources folder, create a file `generator_config.json`

```json
{
  // Version of the images
  "version": 1,
  // Size of the input image (square)
  "input_size": 3000,
  // Size of the output image (square)
  "output_size": 720
}
```

Change the version number each time you want to generate new images. To avoid overwriting existing images.

- In resources folder, create a folder `input`
    - For each layer, create a new folder with the name of the layer. (ex: `background`)
        - In it add a `_meta.json` file that will contain the offsets of the images
      ```json
      {
        // Offset starts top left
        "x_offset": 0,
        "y_offset": 0
      }
    
      ```
        - Then put all images in the folder. (ex: `background/1.png`, `background/2.png`, `background/3.png`)
            - The image name will correspond to the attibutes value (ex: `background/red.png` will give `background: red`)


- Once everything is set up, run `cargo run -p generator` to generate the images in the `output` folder.
    - The images will be names a hash of the attributes. So if anything changes in the file name, it won't overwrite existing images.
        - That's also the reason why the version number is important, as it will change the hash of the attributes.
    - Each image will be associated with a `{hash}.json` file containing the attributes of the image.


## Server

### Goal

- Listen to any mint event on the blockchain, and link a `token_id` to an `image_id`
    - Done in `src/listener.rs`
- Have a web server to be able to get the attributes of an image and th image itself from a `token_id`
    - Done in `src/image.rs`
    - Routes:
        - GET `/data/{token_id}`: Get the metadata of the image
        - GET `/image/{token_id}`: Get the image itself
        - GET `/data`: Debug route to get all the metadata

### Run

#### Environment variables

- `WSS_URL`: The websocket url to listen to

#### Init database

```bash
./resources/db/setup.sh
```

This will create a `app.db` file (sqlite3) in the `resources` folder.

#### Run

Currently e