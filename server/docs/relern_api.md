# relern_api

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
**createAnswer**](relern_api.md#createAnswer) | **POST** /answers | Create a new answer
**health**](relern_api.md#health) | **GET** /health | Health check
**listQuestions**](relern_api.md#listQuestions) | **GET** /questions | List all questions


# **createAnswer**
> models::Answer createAnswer()
Create a new answer

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::Answer**](Answer.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **health**
> models::Health health()
Health check

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::Health**](Health.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listQuestions**
> Vec<models::Question> listQuestions(optional)
List all questions

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i64**| How many items to return at one time (max 100) | 

### Return type

[**Vec<models::Question>**](Question.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

