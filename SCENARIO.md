- Clear image output (rm ./back/resources/output)
- Clear db (rm ./back/resources/app.db)
- Run init db script (./back/resources/db/setup.sh)
- Build images (cd ./back && cargo run --release -p generator)
- Deploy token (cd ./token && ./deploy)
- Start ngrok (ngrok http 8080)
  - Get url
- Go to `https://sepolia.etherscan.io/address/...`
  - Update baseURI with the ngrok url (add /data/ at the end, ex: https://0fa5-2a01-e0a-1a8-bb90-508c-6f11-7a40-3518.ngrok-free.app/data/)
- Update env variables (domain + contract)
- Start server (cd ./back && cargo run --release server)
- Start front app (cd ./front && yarn dev)
  - Go to the inventory page, you'll see that it's empty
- Purchase (min price is 0.0001 ETH)
  - This won't work as the sell is not open
- use the setOpen function (set to true)
- Purchase
  - This will work
  - You should see a log message in the server => event has been received, we are registering the transaction
    - and many logs in ngrok, that means that people are validating the transaction
  - Go back to the inventory page, you should see the new token
