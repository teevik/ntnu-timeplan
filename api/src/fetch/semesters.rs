use crate::error::{AppError, AppResult};
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

use crate::shared_types::{Semester, SemestersWithCurrent};

pub async fn fetch_semesters(client: &Client) -> AppResult<SemestersWithCurrent> {
    let response = client
        .get("https://tp.educloud.no/ntnu/timeplan/timeplan.php?type=courseact")
        .send()
        .await?;

    let html = response.text().await?;

    let document = Html::parse_document(&html);

    let selector = Selector::parse("select#semesterselect option").unwrap();
    let elements = document.select(&selector);

    let result = {
        let mut current_semester = None;
        let mut semesters = HashMap::<String, Semester>::new();

        for element in elements {
            let semester_code = element
                .value()
                .attr("value")
                .ok_or_else(|| AppError::ParsingError)?;
            let semester_name = element
                .text()
                .next()
                .ok_or_else(|| AppError::ParsingError)?;

            if semester_code == "showall" {
                continue;
            }

            let is_current_semester = element.value().attr("selected").is_some();

            let semester = Semester {
                name: semester_name.to_owned(),
            };

            if is_current_semester {
                current_semester = Some(semester_code.to_owned());
            }

            semesters.insert(semester_code.to_owned(), semester);
        }

        let current_semester = current_semester.ok_or_else(|| AppError::ParsingError)?;

        SemestersWithCurrent {
            semesters,
            current_semester,
        }
    };

    Ok(result)
}
