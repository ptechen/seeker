use super::{get_conn, Result};
use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "project";

pub const FIELDS: &str = "project_name,path";

/// Unique：[id]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub project_name: String,
    pub path: String,
}

impl Project {
    pub fn insert(&self) -> Result<usize> {
        let sql = format!("INSERT INTO project ({FIELDS}) VALUES(?1,?2)");
        let data = get_conn().execute(&sql, (&self.project_name, &self.path))?;
        Ok(data)
    }

    pub fn select_all() -> Result<Vec<Self>> {
        let sql = format!("SELECT {FIELDS} FROM {TABLE_NAME} ");
        let conn = get_conn();
        let mut stmt = conn.prepare(&sql)?;
        let person_iter = stmt.query_map([], |row| {
            Ok(Self {
                project_name: row.get(0)?,
                path: row.get(1)?,
            })
        })?;
        let data = person_iter
            .map(|person| person.unwrap())
            .collect::<Vec<_>>();
        Ok(data)
    }

    pub fn select_optional_by_id(id: i64) -> Result<Option<Self>> {
        let sql = format!("SELECT {FIELDS} FROM {TABLE_NAME} WHERE  id = ? ");
        let conn = get_conn();
        let mut stmt = conn.prepare(&sql)?;
        let person_iter = stmt.query_map([], |row| {
            Ok(Self {
                project_name: row.get(0)?,
                path: row.get(1)?,
            })
        })?;
        let data = person_iter
            .map(|person| person.unwrap())
            .collect::<Vec<_>>();
        if data.is_empty() {
            return Ok(None);
        }
        Ok(Some(data[0].clone()))
    }

    pub fn select_one_by_id(id: i64) -> Result<Self> {
        let sql = format!("SELECT {FIELDS} FROM {TABLE_NAME} WHERE  id = ? ");
        let conn = get_conn();
        let mut stmt = conn.prepare(&sql)?;
        let person_iter = stmt.query_map([], |row| {
            Ok(Self {
                project_name: row.get(0)?,
                path: row.get(1)?,
            })
        })?;
        let data = person_iter
            .map(|person| person.unwrap())
            .collect::<Vec<_>>();
        Ok(data[0].clone())
    }
}

// ***************************************以下是自定义代码区域******************************************
/*
example: [
    {"skip_fields": ["updated_at", "created_at"], "filename": "table_name1"},
    {"contain_fields": ["updated_at", "created_at"], "filename": "table_name2"}
]
*/
// *************************************************************************************************

impl Project {
    pub fn create_table() -> Result<()> {
        let sql = r#"
            create table project
            (
                project_name text    not null,
                path         text    not null
            );
        "#;
        let conn = get_conn();
        conn.execute(sql, ())?;
        Ok(())
    }

    pub fn insert_data(&self) -> Result<()> {
        let _ = Self::create_table();
        self.insert()?;
        Ok(())
    }
}

#[test]
fn test_project() {
    let data = Project {
        project_name: "111".to_string(),
        path: "111".to_string(),
    };

    if let Err(e) = data.insert_data() {
        println!("{}", e);
    };
}
