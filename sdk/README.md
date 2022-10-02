# Rust API client for relearnsdk

Relearn SDK

## Overview
- API version: 1.0.0
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `relearnsdk` and add the following to `Cargo.toml` under `[dependencies]`:

```
relearnsdk = { path = "./relearnsdk" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost:8080*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*RelernApi* | [**create_answer**](docs/RelernApi.md#create_answer) | **POST** /answers | Create a new answer
*RelernApi* | [**health**](docs/RelernApi.md#health) | **GET** /health | Health check
*RelernApi* | [**list_questions**](docs/RelernApi.md#list_questions) | **GET** /questions | List all questions


## Documentation For Models

 - [Answer](docs/Answer.md)
 - [Health](docs/Health.md)
 - [Question](docs/Question.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

m@m.com

