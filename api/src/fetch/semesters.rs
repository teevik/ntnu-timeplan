use anyhow::Context;
use reqwest::Client;
use scraper::{Html, Selector};
use ntnu_timeplan_shared::{Semester, SemestersWithCurrent};

pub async fn fetch_semesters(client: &Client) -> anyhow::Result<SemestersWithCurrent> {
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
        let mut semesters = Vec::<Semester>::new();

        for element in elements {
            let semester = element.value().attr("value").context("Parsing error")?;
            let semester_name = element.text().next().context("Parsing error")?;

            let is_current_semester = element.value().attr("selected").is_some();

            let fetched_semester = Semester {
                semester: semester.to_owned(),
                name: semester_name.to_owned(),
            };

            if is_current_semester {
                current_semester = Some(fetched_semester.clone());
            }

            semesters.push(fetched_semester)
        }

        let current_semester = current_semester.context("Parsing error")?;

        SemestersWithCurrent {
            semesters,
            current_semester,
        }
    };

    Ok(result)
}
