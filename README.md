<div style='text-align: center;'>
    <h1>MemoDocs</h1>
</div>
<div style="text-align: center; margin: 20px 0;">
    <img src="local/pepe-black-white-library-shocked.png" style="width: 100%; height: 200; border: 2px solid #000; border-radius: 10px;" />
</div>

**MemoDocs** — это высокопроизводительная база данных Document In-Memory Key-Value Store для оперативного хранения и управления документами. Обеспечивает быстрый доступ и обработку данных благодаря хеш-таблицам.

#### Установка
```
pip install memodocs
```

#### Основные характеристики:
- **Документно-Ориентированная Модель**: Хранение данных в формате документов.
- **Ключ-Значение Хранилище**: Быстрый доступ по уникальным ключам.
- **Временное Хранение**: Все данные хранятся в оперативной памяти.
- **Гибкая Структура**: Поддержка произвольных структур данных.
- **Сохранение Состояния**: Поддержка бэкапов и журналирования.
- **Производительность**: Высокая скорость операций чтения и записи.

#### Возможности:
- **Быстрый доступ**: Мгновенное чтение и запись данных.
- **Удобный интерфейс**: Интуитивно понятный API.
- **Гибкость в работе с данными**: Поддержка сложных структур.
- **Взаимодействие через Python**: Простое управление документами.

#### Преимущества по сравнению с SQLite

| Операция           | MemoDocs (100k operations) | Процентное преимущество |
|--------------------|----------------------------|--------------------------|
| Вставка документов | 0.2610s | 13.3% быстрее              |
| Запрос документов  | 0.1550s | 93.6% быстрее              |
| Удаление документов | 0.1932s | 39.3% быстрее              |

#### Примеры использования:

1. **Создание и добавление документов:**
   ```python
   import json
   from memodocs import DocumentDB

   db = DocumentDB()
   doc1 = {"name": "Alice", "age": 30, "email": "alice@example.com"}
   db.insert("user1", doc1)
   ```

2. **Получение документа:**
   ```python
   retrieved_doc = db.get("user1")
   print("Retrieved document data:", json.dumps(retrieved_doc, indent=2))
   ```

3. **Обновление документа:**
   ```python
   updated_doc = {"name": "Alice", "age": 31, "email": "alice_new@example.com"}
   db.update("user1", updated_doc)
   ```

4. **Удаление документа:**
   ```python
   db.delete("user1")
   ```

5. **Сохранение и загрузка состояния базы данных:**
   ```python
   db.save("backup.db")
   db.load("backup.db")
   ```
