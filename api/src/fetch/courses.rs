use crate::data::course::Course;
use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

pub async fn fetch_courses(client: &Client) -> anyhow::Result<HashMap<String, Course>> {
    let res = client
        .get("https://tp.uio.no/ntnu/timeplan/emner.php")
        .send()
        .await?;

    let page_html = res.text().await?;

    let courses = {
        let (_, courses) = page_html
            .split_once("var courses = ")
            .context("Parsing error")
            .unwrap();

        let end_index = courses.find(']').context("Parsing error")?;

        &courses[0..=end_index]
    };

    #[derive(Debug, Deserialize)]
    struct FetchedCourse {
        #[serde(rename = "id")]
        pub code: String,

        #[serde(rename = "name")]
        pub name: String,

        #[serde(rename = "nofterms")]
        pub amount_of_terms: i32,
    }

    let fetched_courses = serde_json::from_str::<Vec<FetchedCourse>>(courses)?;

    let courses = fetched_courses
        .into_iter()
        .map(|fetched_course| {
            let course_code = fetched_course.code;

            let course = Course {
                name: fetched_course.name,
                amount_of_terms: fetched_course.amount_of_terms,
            };

            (course_code, course)
        })
        .collect::<HashMap<_, _>>();

    Ok(courses)
}
