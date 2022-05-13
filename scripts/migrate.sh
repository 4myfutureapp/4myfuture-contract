#!/bin/bash
echo
echo "Migrating Contract Data..."
echo

near deploy --wasmFile ./out/formyfuture.wasm --initFunction "migrate" --initArgs "{}" --accountId dev-1652466503853-68305726825054