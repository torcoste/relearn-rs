#![allow(unused_qualifications)]

#[cfg(any(feature = "client", feature = "server"))]
use crate::header;
use crate::models;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Answer {
    #[serde(rename = "questionId")]
    pub question_id: i64,

    #[serde(rename = "valid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,

    #[serde(rename = "created_at")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "user_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    #[serde(rename = "org_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
}

impl Answer {
    pub fn new(question_id: i64) -> Answer {
        Answer {
            question_id,
            valid: None,
            created_at: None,
            user_id: None,
            org_id: None,
        }
    }
}

/// Converts the Answer value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Answer {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("questionId".to_string());
        params.push(self.question_id.to_string());

        if let Some(ref valid) = self.valid {
            params.push("valid".to_string());
            params.push(valid.to_string());
        }

        // Skipping created_at in query parameter serialization

        if let Some(ref user_id) = self.user_id {
            params.push("user_id".to_string());
            params.push(user_id.to_string());
        }

        if let Some(ref org_id) = self.org_id {
            params.push("org_id".to_string());
            params.push(org_id.to_string());
        }

        params.join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Answer value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Answer {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub question_id: Vec<i64>,
            pub valid: Vec<bool>,
            pub created_at: Vec<chrono::DateTime<chrono::Utc>>,
            pub user_id: Vec<i64>,
            pub org_id: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Answer".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "questionId" => intermediate_rep.question_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "valid" => intermediate_rep.valid.push(
                        <bool as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "created_at" => intermediate_rep.created_at.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "user_id" => intermediate_rep.user_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "org_id" => intermediate_rep.org_id.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Answer".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Answer {
            question_id: intermediate_rep
                .question_id
                .into_iter()
                .next()
                .ok_or_else(|| "questionId missing in Answer".to_string())?,
            valid: intermediate_rep.valid.into_iter().next(),
            created_at: intermediate_rep.created_at.into_iter().next(),
            user_id: intermediate_rep.user_id.into_iter().next(),
            org_id: intermediate_rep.org_id.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Answer> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Answer>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Answer>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Answer - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Answer> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <Answer as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into Answer - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Health {
    #[serde(rename = "status")]
    pub status: String,
}

impl Health {
    pub fn new(status: String) -> Health {
        Health { status }
    }
}

/// Converts the Health value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Health {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("status".to_string());
        params.push(self.status.to_string());

        params.join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Health value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Health {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub status: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Health".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "status" => intermediate_rep.status.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Health".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Health {
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in Health".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Health> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Health>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Health>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Health - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Health> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <Health as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into Health - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Question {
    #[serde(rename = "level")]
    pub level: String,

    #[serde(rename = "tag")]
    pub tag: String,

    #[serde(rename = "number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,

    #[serde(rename = "question")]
    pub question: String,

    #[serde(rename = "correct_answer")]
    pub correct_answer: i64,

    #[serde(rename = "answers")]
    pub answers: Vec<String>,

    #[serde(rename = "point_reward")]
    pub point_reward: i64,

    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(rename = "hint")]
    pub hint: String,

    #[serde(rename = "reference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<Vec<String>>,

    #[serde(rename = "correct_response")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_response: Option<String>,

    #[serde(rename = "wrong_response")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrong_response: Option<String>,
}

impl Question {
    pub fn new(
        level: String,
        tag: String,
        question: String,
        correct_answer: i64,
        answers: Vec<String>,
        point_reward: i64,
        hint: String,
    ) -> Question {
        Question {
            level,
            tag,
            number: None,
            question,
            correct_answer,
            answers,
            point_reward,
            tags: None,
            hint,
            reference: None,
            correct_response: None,
            wrong_response: None,
        }
    }
}

/// Converts the Question value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Question {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("level".to_string());
        params.push(self.level.to_string());

        params.push("tag".to_string());
        params.push(self.tag.to_string());

        if let Some(ref number) = self.number {
            params.push("number".to_string());
            params.push(number.to_string());
        }

        params.push("question".to_string());
        params.push(self.question.to_string());

        params.push("correct_answer".to_string());
        params.push(self.correct_answer.to_string());

        params.push("answers".to_string());
        params.push(
            self.answers
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",")
                ,
        );

        params.push("point_reward".to_string());
        params.push(self.point_reward.to_string());

        if let Some(ref tags) = self.tags {
            params.push("tags".to_string());
            params.push(
                tags.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
                    ,
            );
        }

        params.push("hint".to_string());
        params.push(self.hint.to_string());

        if let Some(ref reference) = self.reference {
            params.push("reference".to_string());
            params.push(
                reference
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
                    ,
            );
        }

        if let Some(ref correct_response) = self.correct_response {
            params.push("correct_response".to_string());
            params.push(correct_response.to_string());
        }

        if let Some(ref wrong_response) = self.wrong_response {
            params.push("wrong_response".to_string());
            params.push(wrong_response.to_string());
        }

        params.join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Question value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Question {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub level: Vec<String>,
            pub tag: Vec<String>,
            pub number: Vec<i64>,
            pub question: Vec<String>,
            pub correct_answer: Vec<i64>,
            pub answers: Vec<Vec<String>>,
            pub point_reward: Vec<i64>,
            pub tags: Vec<Vec<String>>,
            pub hint: Vec<String>,
            pub reference: Vec<Vec<String>>,
            pub correct_response: Vec<String>,
            pub wrong_response: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing Question".to_string(),
                    )
                }
            };

            if let Some(key) = key_result {
                match key {
                    "level" => intermediate_rep.level.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "tag" => intermediate_rep.tag.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "number" => intermediate_rep.number.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "question" => intermediate_rep.question.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "correct_answer" => intermediate_rep.correct_answer.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "answers" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Question"
                                .to_string(),
                        )
                    }
                    "point_reward" => intermediate_rep.point_reward.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?,
                    ),
                    "tags" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Question"
                                .to_string(),
                        )
                    }
                    "hint" => intermediate_rep.hint.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "reference" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in Question"
                                .to_string(),
                        )
                    }
                    "correct_response" => intermediate_rep.correct_response.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    "wrong_response" => intermediate_rep.wrong_response.push(
                        <String as std::str::FromStr>::from_str(val)
                            .map_err(|x| format!("{}", x))?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing Question".to_string(),
                        )
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Question {
            level: intermediate_rep
                .level
                .into_iter()
                .next()
                .ok_or_else(|| "level missing in Question".to_string())?,
            tag: intermediate_rep
                .tag
                .into_iter()
                .next()
                .ok_or_else(|| "tag missing in Question".to_string())?,
            number: intermediate_rep.number.into_iter().next(),
            question: intermediate_rep
                .question
                .into_iter()
                .next()
                .ok_or_else(|| "question missing in Question".to_string())?,
            correct_answer: intermediate_rep
                .correct_answer
                .into_iter()
                .next()
                .ok_or_else(|| "correct_answer missing in Question".to_string())?,
            answers: intermediate_rep
                .answers
                .into_iter()
                .next()
                .ok_or_else(|| "answers missing in Question".to_string())?,
            point_reward: intermediate_rep
                .point_reward
                .into_iter()
                .next()
                .ok_or_else(|| "point_reward missing in Question".to_string())?,
            tags: intermediate_rep.tags.into_iter().next(),
            hint: intermediate_rep
                .hint
                .into_iter()
                .next()
                .ok_or_else(|| "hint missing in Question".to_string())?,
            reference: intermediate_rep.reference.into_iter().next(),
            correct_response: intermediate_rep.correct_response.into_iter().next(),
            wrong_response: intermediate_rep.wrong_response.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Question> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Question>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<Question>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for Question - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Question> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <Question as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        "Unable to convert header value '{}' into Question - {}",
                        value, err
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Unable to convert header: {:?} to string: {}",
                hdr_value, e
            )),
        }
    }
}
