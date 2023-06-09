# How to test

- First install everything, and have the server running
- If you are using the same contract as me go
  to [https://sepolia.etherscan.io/address/0x6a07fa991081d4884a69d5feb22aeb1411cbdb78#writeContract](https://sepolia.etherscan.io/address/0x7fF28E2Cf335A8844Cc84B16114DB7Af740aAe33#writeContract)
    - Connect your wallet
    - Purchase a token 💰 (1 token = 0.000001 eth or use the `DEBUG_STEAL` method)
    - And that's it, if everything works you have a new token in your wallet 🎉 with a magnificent image 🖼️
        - You can also enjoy many people validating your transaction on the blockchain :
      ```bash
      [2023-06-01T14:42:12Z INFO  server::listener] New transfer: to: 0xc5bf8612427bbfb6b1c7061d9a18873c2a27e81f, token_id: 5
      [2023-06-01T14:42:12Z INFO  server::listener] Assigned image 355639902cdded713c8db35bf5d4c0b5f7da3f91eca1897a130cb9d4da603b3e to 0xc5bf8612427bbfb6b1c7061d9a18873c2a27e81f with token id 5
      [2023-06-01T14:42:22Z INFO  actix_web::middleware::logger] 127.0.0.1 "HEAD /data/5 HTTP/1.1" 404 0 "-" "got/9.6.0 (https://github.com/sindresorhus/got)" 0.000127
      [2023-06-01T14:42:22Z INFO  actix_web::middleware::logger] 127.0.0.1 "GET /data/5 HTTP/1.1" 200 544 "-" "got/9.6.0 (https://github.com/sindresorhus/got)" 0.001278
      [2023-06-01T14:42:23Z INFO  actix_web::middleware::logger] 127.0.0.1 "GET /data/5 HTTP/1.1" 200 544 "-" "python-requests/2.30.0" 0.000790
      [2023-06-01T14:42:23Z INFO  actix_web::middleware::logger] 127.0.0.1 "HEAD /data/5 HTTP/1.1" 404 0 "-" "python-requests/2.30.0" 0.000194
      [2023-06-01T14:42:23Z INFO  actix_web::middleware::logger] 127.0.0.1 "GET /data/5 HTTP/1.1" 200 544 "-" "python-requests/2.30.0" 0.000810
      [2023-06-01T14:42:24Z INFO  actix_web::middleware::logger] 127.0.0.1 "HEAD /image/5 HTTP/1.1" 404 0 "-" "python-requests/2.30.0" 0.000061
      [2023-06-01T14:42:24Z INFO  actix_web::middleware::logger] 127.0.0.1 "GET /image/5 HTTP/1.1" 200 224013 "-" "python-requests/2.30.0" 0.001883
      [2023-06-01T14:42:25Z INFO  actix_web::middleware::logger] 127.0.0.1 "HEAD /data/5 HTTP/1.1" 404 0 "-" "axios/0.26.1" 0.000121
      [2023-06-01T14:42:26Z INFO  actix_web::middleware::logger] 127.0.0.1 "GET /data/5 HTTP/1.1" 200 544 "-" "axios/0.26.1" 0.001247
      [2023-06-01T14:42:28Z INFO  actix_web::middleware::logger] 127.0.0.1 "HEAD /data/5 HTTP/1.1" 404 0 "-" "got/9.6.0 (https://github.com/sindresorhus/got)" 0.000122
      [2023-06-01T14:42:28Z INFO  actix_web::middleware::logger] 127.0.0.1 "GET /data/5 HTTP/1.1" 200 544 "-" "got/9.6.0 (https://github.com/sindresorhus/got)" 0.000322
      ```
    - From what I see and what I read on internet nft images are not showing in sepolia.etherscan, however you can call
      the `tokenURI` function with your token id to get the token metadata, and the `image` field will contain the image
      url.

# Installation

_tl;dr: install rust, copy `.envrc.example` to `.envrc` and fill it, go to `back` folder run then `cargo run --release -p generator` and `cargo run --release -p server`_

## Token

The idea is:

- 1 purchase per day / person => `mintedOnDayUser`
- maximum of 100 buyers per day => `mintedOnDay`
- ability to activate or not the sale (in case we want to update the backend)

For testing purpose I've added a `DEBUG_STEAL` function that allows to mint a token for free and bypass the `mintedOnDayUser`
and `mintedOnDay` checks.

### Deploy with forge

Use the `deploy.sh` script to deploy the contract.

Once deployed you have to run the `setBaseURI` function to set the base uri of the token metadata. (I personnaly use
ngrok to point at my local server)
If you plan to use the `purchase` function, you also have to run the `setOpen` function to allow the purchase.

## Back

The backend is build in rust, and is composed of 2 parts:

- image generator (generator)
- server

You can run each project individually using `cargo run -p {project_name}`, or build them
using `cargo build -p {project_name}` then run the binary in `target/debug/{project_name}`

### Image Generator

_Not the best code, but as it's goal is to be run only once, it's ok._

#### How to use

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
            - The image name will correspond to the attibutes value (ex: `background/red.png` will
              give `background: red`)


- Once everything is set up, run `cargo run -p generator` to generate the images in the `output` folder.
    - The image's name will be a hash of the attributes. So if anything changes in the file name, it won't overwrite
      existing images.
        - That's also the reason why the version number is important, as it will change the hash of the attributes.
    - Each image will be associated with a `{hash}.json` file containing the attributes of the image.

### Server

#### Goal

- Listen to any mint event on the blockchain, and link a `token_id` to an `image_id`
    - Done in `src/listener.rs`
- Have a web server to be able to get the attributes of an image and th image itself from a `token_id`
    - Done in `src/image.rs`
    - Routes:
        - GET `/data/{token_id}`: Get the metadata of the image
        - GET `/image/{token_id}`: Get the image itself
        - GET `/data`: Debug route to get all the metadata

#### Run

##### Environment variables

- `ALCHEMY_WSS_URL`: The websocket url to listen to

##### Init database

```bash
./resources/db/setup.sh
```

This will create a `app.db` file (sqlite3) in the `resources` folder.

---

You can now run the server with `cargo run -p server`