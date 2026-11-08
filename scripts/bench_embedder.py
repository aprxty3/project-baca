#!/usr/bin/env python3
"""Measures the embedding service the way Rotaria uses it: reader queries one
at a time, then book passages one per request with queries arriving between
them (ARCHITECTURE.md section 8).

usage: python3 scripts/bench_embedder.py [--url URL] [--container NAME]

On the production host the embedder has no published port; measure a copy
of it on the same model volume, limited like the service:

    docker run -d --name rotaria_bench --cpus 1 --memory 3g -p 127.0.0.1:8099:80 \\
        -v <project>_embedder_models:/data:ro <EMBEDDER_IMAGE> \\
        --model-id /data/embeddinggemma-300m-q4 --pooling mean --tokenization-workers 1
    python3 scripts/bench_embedder.py --url http://127.0.0.1:8099 --container rotaria_bench
    docker rm -f rotaria_bench

Standard library only, so it runs on a bare host.
"""
import argparse
import json
import os
import statistics
import subprocess
import threading
import time
import urllib.request

# The prompts crates/infra/src/ai/embedding.rs and python_worker/worker.py send.
QUERY_PREFIX = "task: search result | query: "
PASSAGE_PREFIX = "title: none | text: "
QUERIES = [
    "kabut tebal menyelimuti pelabuhan saat kapal berangkat",
    "the captain writes down every evening what the stranger told him",
    "ia menunda pekerjaannya dan menghabiskan hari di danau",
    "a letter arrives that changes the whole family's plans",
]
SENTENCE = ("Pagi itu kabut turun dari bukit dan menutup jalan ke pelabuhan, "
            "sementara para nelayan menunggu angin berganti arah sebelum berangkat. ")


def embed(url: str, text: str) -> float:
    body = json.dumps({"inputs": [text], "normalize": True, "truncate": True}).encode()
    request = urllib.request.Request(url + "/embed", data=body, headers={"content-type": "application/json"})
    start = time.perf_counter()
    with urllib.request.urlopen(request, timeout=300) as response:
        vectors = json.load(response)
    assert len(vectors) == 1 and len(vectors[0]) == 768, "expected one 768-dimension vector"
    return time.perf_counter() - start


def percentile(values: list[float], share: float) -> float:
    ordered = sorted(values)
    return ordered[min(len(ordered) - 1, int(share * len(ordered)))]


def report(label: str, seconds: list[float]) -> None:
    ms = [s * 1000 for s in seconds]
    print(f"{label}: n={len(ms)} p50 {statistics.median(ms):.0f} ms, p95 {percentile(ms, 0.95):.0f} ms")


def memory(container: str | None) -> str:
    if not container:
        return "n/a (pass --container)"
    out = subprocess.run(["docker", "stats", "--no-stream", "--format", "{{.MemUsage}}", container],
                         capture_output=True, text=True, check=False)
    return out.stdout.strip() or out.stderr.strip()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--url", default=os.environ.get("EMBEDDING_URL", "http://127.0.0.1:8090"))
    parser.add_argument("--container", help="docker container to read memory from")
    parser.add_argument("--queries", type=int, default=100)
    parser.add_argument("--passages", type=int, default=20)
    args = parser.parse_args()
    url = args.url.rstrip("/")

    embed(url, QUERY_PREFIX + "pemanasan")
    report("query, sequential", [embed(url, QUERY_PREFIX + QUERIES[i % len(QUERIES)]) for i in range(args.queries)])

    passage = PASSAGE_PREFIX + " ".join((SENTENCE * 20).split()[:300])
    during: list[float] = []
    done = threading.Event()

    def readers() -> None:
        while not done.is_set():
            during.append(embed(url, QUERY_PREFIX + QUERIES[len(during) % len(QUERIES)]))
            time.sleep(0.5)

    thread = threading.Thread(target=readers)
    start = time.perf_counter()
    thread.start()
    passage_times = [embed(url, passage) for _ in range(args.passages)]
    done.set()
    thread.join()
    elapsed = time.perf_counter() - start
    report("300-word passage", passage_times)
    print(f"passages per second (with queries between): {args.passages / elapsed:.2f}")
    report("query while passages embed", during)
    print(f"memory: {memory(args.container)}")


if __name__ == "__main__":
    main()
