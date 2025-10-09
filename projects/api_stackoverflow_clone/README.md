<a id="readme-top"></a>

<!-- Project Title -->
<br>
<div>
    <h1 style="text-align:center">API Stack Overflow Clone</h1>
</div>

<!-- Table of Contents -->
<details>
    <summary>Table of Contents</summary>
    <ol>
        <li><a href="#project-details">Project Details</a></li>
        <li><a href="#using-the-api">Using the API</a></li>
    </ol>
</details>
<br>

<!-- Project Details -->
## Project Details
A clone of the Stack Overflow API. This project allows a person to create new
questions and post answers to the questions. There is also the option to remove a question and or answer.

This project was creating using the Axum crate and utilized the Postgres SQL server.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- Setup -->
## Setup
1. Create Postgres SQL by using Docker. Once you have installed Docker onto your computer run the following command
in your terminal to pull the Postgres container image.
   

``Bash
docker pull postgres
``

Next is to create a Postgres SQL instance by running the following command in your terminal.

``Bash
docker run --name stack-overflow-db -e POSTGRES_PASSWORD=postgrespw -p 55008:5432 -d postgres
``

2. With the Docker Postgres SQL container running, install the SQLx's associated command-line utility for managing
databases, migrations, and more.

``Bash
cargo install sqlx-cli
``

Use the following command to execute migrations.

``Bash
sqlx migrate run
``

If you need to revert the migrations, use the following command.

``Bash
sqlx migrate revert
``

<!-- Using the API -->
## Using the API
With the Postgres SQL Docker container running and the Rust program running, run the following cURL commands in the
terminal to validate that everything is running correctly.

Create question

```text
curl --location 'localhost:8000/question' \
--header 'Content-Type: application/json' \
--data '{
    "title": "Newly Created Question",
    "description": "My Description"
}'
```

Get questions

```text
curl --location 'localhost:8000/questions'
```

Create answer

```text
curl --location 'localhost:8000/answer' \
--header 'Content-Type: application/json' \
--data '{ 
    "question_uuid": "[UUID of a created question]",
    "content": "test question"
}'
```

Get answers

```text
curl --location --request GET 'localhost:8000/answers' \
--header 'Content-Type: application/json' \
--data '{
    "question_uuid": "[UUID of a created question]"
}'
```

Delete answer

```text
curl --location --request DELETE 'localhost:8000/answer' \
--header 'Content-Type: application/json' \
--data '{ 
    "answer_uuid": "[UUID of a created answer]"
}'
```

Delete question

```text
curl --location --request DELETE 'localhost:8000/question' \
--header 'Content-Type: application/json' \
--data '{ 
    "question_uuid": "[UUID of a created question]"
}'
```


<p align="right">(<a href="#readme-top">back to top</a>)</p>