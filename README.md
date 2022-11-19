TEST:
cargo test -- --nocapture

BUILD:
cargo build --target wasm32-unknown-unknown -–release

DEPLOY:
near deploy --wasmFile target/wasm32-unknown-unknown/release/Score.wasm --accountId YOUR_ACCOUNT_HERE

CALL METHODS FROM COMMAND LINE:

GENERIC EXAMPLE:
near call CONTRACT_ACCOUNT_HERE method_name '{"key":"input"}' --accountId CALLING_ACCOUNT_HERE --amount 1 --gas 10000000000000

THIS CONTRACT EXAMPLE CALLS:

// Shortcut so you don't have to type out something like: score.myaccount.testnet every time
export ID=CONTRACT_ACCOUNT_HERE

near call $ID set_size_of_leaderboard '{"size":10}' --accountId $ID

near view $ID get_size_of_leaderboard

near call $ID insert_score '{"account_id":"someone.testnet", "value": 100}' --accountId $ID

near call $ID insert_leaderboard '{"account_id":"someone.testnet", "value": 100}' --accountId $ID

near view $ID get_leaderboard

near view $ID get_score '{"account_id":"someone.testnet"}'

// NOTE: need to have someone.testnet's wallet key / logged in

near call $ID add_self_to_submit --accountId someone.testnet --amount 0.01   

near view $ID users_waiting_to_submit

near view $ID get_users_to_submit

near call $ID clear_user_to_submit '{"account_id" : "someone.testnet"}' --accountId $ID

near call $ID reset_user_score '{"account_id":"someone.testnet"}' --accountId $ID

near call $ID remove_leaderboard '{"account_id":"someone.testnet"}' --accountId $ID

