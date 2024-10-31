import bz2
import gzip
import lzma
import time
import zlib

results = {}

def compress_zlib_max(data):
    return zlib.compress(data, 9)

def compress_zlib_min(data):
    return zlib.compress(data, 1)

def compress_gzip_max(data):
    return gzip.compress(data, 5)

def compress_gzip_min(data):
    return gzip.compress(data, 1)

def compress_lzma(data):
    return lzma.compress(data)

def compress_bzip2_max(data):
    return bz2.compress(data, 9)

def compress_bzip2_min(data):
    return bz2.compress(data, 1)

def compress_none(data):
    return data


with open("../.etc/codec.nbt", "rb") as f:
    data = f.read()
    original_size = len(data)
    for algo in [compress_zlib_max, compress_zlib_min, compress_gzip_max, compress_gzip_min, compress_lzma, compress_bzip2_max, compress_bzip2_min, compress_none]:
        print(f"Testing {algo.__name__}...")
        times = []
        sizediff = 0
        for iteration in range(1, 3000):
            start = time.time_ns()
            compressed = algo(data)
            end = time.time_ns()
            times.append(end - start)
            sizediff = len(compressed) / original_size
        average_time = (sum(times) / len(times) / 1000) / 1000
        results[algo.__name__] = {
            "average_time": average_time,
            "size_decrease_percentage": sizediff
        }
    for result in results:
        name = result
        average_time = results[result]["average_time"]
        sizediff = results[result]["size_decrease_percentage"]
        print(f"Algorithm: {name}")
        print(f"Average Time: {average_time:.3f} ms")
        print(f"Compressed Size: {sizediff*100:.2f}% of original size")
        print()