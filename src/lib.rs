use std::collections::HashMap;
use std::fs::{File, rename};
use std::io::{Read, Write};
use std::path::Path;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use pyo3::exceptions::{PyOSError, PyValueError};

/// A simple document-oriented in-memory database with persistent storage capabilities.
///
/// This class provides a basic document store that can be used to insert, 
/// retrieve, update, delete, and persist documents (represented as dictionaries) to disk.
///
/// Attributes:
///     data (dict): A dictionary storing documents (key-value pairs) in memory.
///     file_path (str): The path to the file where the database is saved.
#[pyclass]
pub struct DocumentDB {
    data: HashMap<String, PyObject>,
    file_path: String,
}

#[pymethods]
impl DocumentDB {
    /// Creates a new `DocumentDB` instance.
    ///
    /// # Arguments
    ///
    /// * `file_path` (str): Path to the file where data will be saved.
    ///
    /// # Returns
    ///
    /// `DocumentDB`: A new instance of the `DocumentDB` class.
    ///
    /// # Example
    ///
    /// ```python
    /// db = DocumentDB("data.db")
    /// ```
    #[new]
    pub fn new(file_path: &str) -> Self {
        DocumentDB {
            data: HashMap::new(),
            file_path: file_path.to_string(),
        }
    }

    /// Inserts a document into the database.
    ///
    /// # Arguments
    ///
    /// * `doc_id` (str): The identifier for the document.
    /// * `document` (dict): The document to insert.
    ///
    /// # Example
    ///
    /// ```python
    /// db.insert("doc1", {"name": "Alice", "age": 30})
    /// ```
    pub fn insert(&mut self, py: Python, doc_id: String, document: &PyDict) {
        self.data.insert(doc_id, document.to_object(py));
    }

    /// Retrieves a document by its identifier.
    ///
    /// # Arguments
    ///
    /// * `py` (Python): The Python interpreter context.
    /// * `doc_id` (str): The identifier of the document to retrieve.
    ///
    /// # Returns
    ///
    /// `PyResult[Optional[dict]]`: The document associated with the identifier, or `None` if the identifier does not exist.
    ///
    /// # Example
    ///
    /// ```python
    /// document = db.get("doc1")
    /// ```
    pub fn get(&self, py: Python, doc_id: &str) -> PyResult<Option<PyObject>> {
        Ok(self.data.get(doc_id).cloned())
    }

    /// Retrieves all documents stored in the database.
    ///
    /// # Returns
    ///
    /// `PyResult[dict]`: A dictionary containing all documents.
    ///
    /// # Example
    ///
    /// ```python
    /// all_docs = db.get_all()
    /// ```
    pub fn get_all(&self, py: Python) -> PyResult<HashMap<String, PyObject>> {
        let mut result = HashMap::new();
        for (key, value) in &self.data {
            result.insert(key.clone(), value.clone());
        }
        Ok(result)
    }

    /// Deletes a document by its identifier.
    ///
    /// # Arguments
    ///
    /// * `doc_id` (str): The identifier of the document to delete.
    ///
    /// # Example
    ///
    /// ```python
    /// db.delete("doc1")
    /// ```
    pub fn delete(&mut self, doc_id: &str) {
        self.data.remove(doc_id);
    }

    /// Updates a document by its identifier.
    ///
    /// # Arguments
    ///
    /// * `doc_id` (str): The identifier of the document to update.
    /// * `document` (dict): The new document to associate with the identifier.
    ///
    /// # Example
    ///
    /// ```python
    /// db.update("doc1", {"name": "Bob", "age": 25})
    /// ```
    pub fn update(&mut self, py: Python, doc_id: String, document: &PyDict) {
        if self.data.contains_key(&doc_id) {
            self.data.insert(doc_id, document.to_object(py));
        }
    }

    /// Saves the current state of the database to disk.
    ///
    /// This method writes the data to a temporary file and then renames it to
    /// the file specified by `file_path`. If the file operations fail, an
    /// `OSError` will be raised.
    ///
    /// # Returns
    ///
    /// `PyResult[None]`: Indicates success or failure of the operation.
    ///
    /// # Example
    ///
    /// ```python
    /// db.save()
    /// ```
    pub fn save(&self) -> PyResult<()> {
        let temp_path = Path::new(&self.file_path).with_extension("tmp");
        let mut temp_file = File::create(&temp_path).map_err(|e| PyErr::new::<PyOSError, _>(e.to_string()))?;

        for (doc_id, doc) in &self.data {
            let id_len = doc_id.len() as u64;
            let doc_str = doc.to_string();
            let doc_bytes = doc_str.into_bytes();
            let doc_len = doc_bytes.len() as u64;

            temp_file.write_all(&id_len.to_le_bytes()).map_err(|e| PyErr::new::<PyOSError, _>(e.to_string()))?;
            temp_file.write_all(doc_id.as_bytes()).map_err(|e| PyErr::new::<PyOSError, _>(e.to_string()))?;
            temp_file.write_all(&doc_len.to_le_bytes()).map_err(|e| PyErr::new::<PyOSError, _>(e.to_string()))?;
            temp_file.write_all(&doc_bytes).map_err(|e| PyErr::new::<PyOSError, _>(e.to_string()))?;
        }

        rename(&temp_path, &self.file_path).map_err(|e| PyErr::new::<PyOSError, _>(e.to_string()))?;
        Ok(())
    }

    /// Loads the state of the database from disk.
    ///
    /// This method reads data from the file specified by `file_path`. If the
    /// file does not exist, the method does nothing. If file operations or
    /// data deserialization fail, an `OSError` or `ValueError` will be raised.
    ///
    /// # Arguments
    ///
    /// * `py` (Python): The Python interpreter context.
    ///
    /// # Returns
    ///
    /// `PyResult[None]`: Indicates success or failure of the operation.
    ///
    /// # Example
    ///
    /// ```python
    /// db.load()
    /// ```
    pub fn load(&mut self, py: Python) -> PyResult<()> {
        let path = Path::new(&self.file_path);
        if !path.exists() {
            return Ok(());
        }

        let mut file = File::open(path).map_err(|e| PyErr::new::<PyOSError, _>(e.to_string()))?;
        self.data.clear();

        let mut buffer = [0; 8];
        while let Ok(_) = file.read_exact(&mut buffer) {
            let id_len_bytes = buffer;
            let id_len = u64::from_le_bytes(id_len_bytes);

            let mut id_bytes = vec![0; id_len as usize];
            file.read_exact(&mut id_bytes).map_err(|e| PyErr::new::<PyOSError, _>(e.to_string()))?;
            let doc_id = String::from_utf8(id_bytes).map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))?;

            let mut doc_len_bytes = [0; 8];
            file.read_exact(&mut doc_len_bytes).map_err(|e| PyErr::new::<PyOSError, _>(e.to_string()))?;
            let doc_len = u64::from_le_bytes(doc_len_bytes);

            let mut doc_bytes = vec![0; doc_len as usize];
            file.read_exact(&mut doc_bytes).map_err(|e| PyErr::new::<PyOSError, _>(e.to_string()))?;

            let doc_str = String::from_utf8(doc_bytes).map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))?;
            let doc = PyString::new(py, &doc_str).into();

            self.data.insert(doc_id, doc);
        }

        if let Err(e) = file.read_exact(&mut buffer) {
            if e.kind() != std::io::ErrorKind::UnexpectedEof {
                return Err(PyErr::new::<PyOSError, _>(e.to_string()));
            }
        }

        Ok(())
    }
}


#[pymodule]
fn memodocs(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DocumentDB>()?;
    Ok(())
}
