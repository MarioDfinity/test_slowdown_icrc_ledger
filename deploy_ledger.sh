#!/usr/bin/env bash

set -euo pipefail
set -x

ARG='(variant { Init=record {
    minting_account=record {owner=principal "aspvh-rnqud-zk2qo-4objq-d537b-j36qa-l74s3-jricy-57syn-tt5iq-bae"};
    transfer_fee=0:nat;
    token_symbol="FOO";
    token_name="The Foo Token";
    metadata=vec {};
    initial_balances=vec {
        record {
            record {
                owner=principal "imx2d-dctwe-ircfz-emzus-bihdn-aoyzy-lkkdi-vi5vw-npnik-noxiy-mae";
            };
            1_000_000_000_000:nat;
        };
    };
    archive_options=record {
        num_blocks_to_archive=1:nat64;
        trigger_threshold=1_000_000_000:nat64;
        controller_id=principal "aspvh-rnqud-zk2qo-4objq-d537b-j36qa-l74s3-jricy-57syn-tt5iq-bae";
    };
}})'

dfx stop
dfx start --background --clean
dfx deploy ledger \
    --argument "$ARG"
