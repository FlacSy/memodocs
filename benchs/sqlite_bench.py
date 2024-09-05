import time
import sqlite3

def create_table(cursor):
    cursor.execute('''
    CREATE TABLE IF NOT EXISTS documents (
        key TEXT PRIMARY KEY,
        field INTEGER
    )
    ''')

def benchmark_insert(db, num_docs):
    start_time = time.time()
    cursor = db.cursor()
    create_table(cursor)
    for i in range(num_docs):
        cursor.execute('INSERT INTO documents (key, field) VALUES (?, ?)', (f'key_{i}', i))
    db.commit()
    end_time = time.time()
    print(f"Time taken to insert {num_docs} documents: {end_time - start_time:.4f} seconds")

def benchmark_query(db, num_docs):
    start_time = time.time()
    cursor = db.cursor()
    for i in range(num_docs):
        cursor.execute('SELECT field FROM documents WHERE key = ?', (f'key_{i}',))
        cursor.fetchone()
    end_time = time.time()
    print(f"Time taken to query {num_docs} documents: {end_time - start_time:.4f} seconds")

def benchmark_delete(db, num_docs):
    start_time = time.time()
    cursor = db.cursor()
    for i in range(num_docs):
        cursor.execute('DELETE FROM documents WHERE key = ?', (f'key_{i}',))
    db.commit()
    end_time = time.time()
    print(f"Time taken to delete {num_docs} documents: {end_time - start_time:.4f} seconds")

if __name__ == "__main__":
    db = sqlite3.connect('test_sqlite.db')
    num_docs = 100000

    benchmark_insert(db, num_docs)
    benchmark_query(db, num_docs)
    benchmark_delete(db, num_docs)

    db.close()
