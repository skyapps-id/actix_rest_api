### CURL
1. Get ALL
    ```sh
    $ curl -X GET http://127.0.0.1:8080/books
    ```
2. Get Once
    ```sh
    $ curl -X GET http://127.0.0.1:8080/books/{id}
    ```
3. Update
    ```sh
    $ curl -X PUT http://127.0.0.1:8080/books/42e3f9b5-2483-4793-94f4-19b80e5ae13a -H "Content-Type: application/json" -d '{"title": "Updated Book Title", "author": "Existing Author Name", "published_year": 2021}'
    ```
4. Delete
    ```sh
    $ curl -X DELETE http://127.0.0.1:8080/books/42e3f9b5-2483-4793-94f4-19b80e5ae13a -H "Content-Type: application/json"
    ```