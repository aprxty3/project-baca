#!/usr/bin/env bash
set -euo pipefail

# Downloads the embedding model at a pinned revision into MODEL_DIR, once.
# The compose `embedder-fetch` service runs it before `embedder` starts, so
# the model service itself needs no route out (it sits on `store` only) and
# a restart never depends on huggingface.co. A rerun with the files in
# place downloads nothing; every file the service reads, tokenizer and
# configuration included, is checked against SHA-256.
# text-embeddings-inference serves onnx/model.onnx with ONNX Runtime and
# reads its `sentence_embedding` output (pooling, dense layers and
# normalisation inside the graph). The 4-bit export ranks quotes as well as
# float32 on our query set (hit@3 0.88 against 0.87) at a sixth of the size;
# it keeps its weights in model_q4.onnx_data, the name the graph refers to.

MODEL_REPO="${MODEL_REPO:-onnx-community/embeddinggemma-300m-ONNX}"
MODEL_REVISION="${MODEL_REVISION:-5090578d9565bb06545b4552f76e6bc2c93e4a66}"
MODEL_DIR="${MODEL_DIR:-/data/embeddinggemma-300m-q4}"
# local path=repository path
FILES=(config.json=config.json tokenizer.json=tokenizer.json
    tokenizer_config.json=tokenizer_config.json
    special_tokens_map.json=special_tokens_map.json
    onnx/model.onnx=onnx/model_q4.onnx
    onnx/model_q4.onnx_data=onnx/model_q4.onnx_data)
CHECKSUMS="6e1f06404b7163e0325ed2ea3e6781cde50f4a50b31780a95ad0d30e8404d77b  config.json
4dda02faaf32bc91031dc8c88457ac272b00c1016cc679757d1c441b248b9c47  tokenizer.json
3ca953eea6c3c9fcda9cf3df22949ff18b216f7c74bd6459230f3f1013953f3a  tokenizer_config.json
2f7b0adf4fb469770bb1490e3e35df87b1dc578246c5e7e6fc76ecf33213a397  special_tokens_map.json
ad1dfee81a70f7944b9b9d1cc6e48075b832881cf33fab2f2b248be78f3f0043  onnx/model.onnx
599962c3143b040de2dd05e5975be3e9091dd067cacc6a8f7186e3203bab9e02  onnx/model_q4.onnx_data"

stamp="$MODEL_DIR/.revision"
complete() {
    [[ "$(cat "$stamp" 2>/dev/null)" == "$MODEL_REVISION" ]] || return 1
    for file in "${FILES[@]}"; do [[ -s "$MODEL_DIR/${file%%=*}" ]] || return 1; done
}
if complete; then
    echo "model $MODEL_REPO@$MODEL_REVISION already in $MODEL_DIR"
    exit 0
fi

mkdir -p "$MODEL_DIR/onnx"
for file in "${FILES[@]}"; do
    local_path="${file%%=*}"
    echo "fetching ${file#*=}"
    curl -fsSL --retry 3 -o "$MODEL_DIR/$local_path.part" \
        "https://huggingface.co/$MODEL_REPO/resolve/$MODEL_REVISION/${file#*=}"
    mv "$MODEL_DIR/$local_path.part" "$MODEL_DIR/$local_path"
done
(cd "$MODEL_DIR" && echo "$CHECKSUMS" | sha256sum -c -)
echo "$MODEL_REVISION" > "$stamp"
echo "model $MODEL_REPO@$MODEL_REVISION ready in $MODEL_DIR"
