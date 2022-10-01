# \RelernApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_answer**](RelernApi.md#create_answer) | **POST** /answers | Create a new answer
[**health**](RelernApi.md#health) | **GET** /health | Health check
[**list_questions**](RelernApi.md#list_questions) | **GET** /questions | List all questions



## create_answer

> crate::models::Answer create_answer()
Create a new answer

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Answer**](Answer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health

> crate::models::Health health()
Health check

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Health**](Health.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_questions

> Vec<crate::models::Question> list_questions(limit)
List all questions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | How many items to return at one time (max 100) |  |

### Return type

[**Vec<crate::models::Question>**](Question.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

