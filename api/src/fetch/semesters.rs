use crate::data::semester::Semester;
use anyhow::Context;
use reqwest::Client;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct FetchedSemester {
    pub semester: Semester,
    pub name: String,
}

#[derive(Debug)]
pub struct FetchedSemesters {
    pub semesters: Vec<FetchedSemester>,
    pub primary_semester: Semester,
}

pub async fn fetch_semesters(client: &Client) -> anyhow::Result<FetchedSemesters> {
    let response = client
        .get("https://tp.educloud.no/ntnu/timeplan/timeplan.php?type=courseact")
        .send()
        .await?;

    let html = response.text().await?;

    let document = Html::parse_document(&html);

    let selector = Selector::parse("select#semesterselect option").unwrap();
    let elements = document.select(&selector);

    let result = {
        let mut primary_semester = None;
        let mut semesters = Vec::<FetchedSemester>::new();

        for element in elements {
            let semester = element.value().attr("value").context("Parsing error")?;
            let semester_name = element.text().next().context("Parsing error")?;

            let is_primary = element.value().attr("selected").is_some();

            if is_primary {
                primary_semester = Some(Semester(semester.to_owned()));
            }

            semesters.push(FetchedSemester {
                semester: Semester(semester.to_owned()),
                name: semester_name.to_owned(),
            })
        }

        let primary_semester = primary_semester.context("Parsing error")?;

        FetchedSemesters {
            semesters,
            primary_semester,
        }
    };

    Ok(result)
}
