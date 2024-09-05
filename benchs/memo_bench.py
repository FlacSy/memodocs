import time
from memodocs import DocumentDB


def benchmark_insert(db, num_docs):
    start_time = time.time()
    for i in range(num_docs):
        db.insert(f"key_{i}",{"field": i})
    end_time = time.time()
    print(f"Time taken to insert {num_docs} documents: {end_time - start_time:.4f} seconds")

def benchmark_query(db, num_docs):
    start_time = time.time()
    for i in range(num_docs):
        db.get(f"key_{i}")
    end_time = time.time()
    print(f"Time taken to query {num_docs} documents: {end_time - start_time:.4f} seconds")

def benchmark_delete(db, num_docs):
    start_time = time.time()
    for i in range(num_docs):
        db.delete(f"key_{i}")
    end_time = time.time()
    print(f"Time taken to delete {num_docs} documents: {end_time - start_time:.4f} seconds")

if __name__ == "__main__":
    db = DocumentDB('db.dbd')
    num_docs = 100000

    benchmark_insert(db, num_docs)
    benchmark_query(db, num_docs)
    benchmark_delete(db, num_docs)
    db.save_to_disk()
