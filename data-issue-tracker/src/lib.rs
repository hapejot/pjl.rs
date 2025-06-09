// This file defines the public API of the "data issue tracker" library.

pub struct Issue {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: IssueStatus,
}

pub enum IssueStatus {
    Open,
    InProgress,
    Closed,
}

impl Issue {
    pub fn new(id: u32, title: String, description: String) -> Self {
        Issue {
            id,
            title,
            description,
            status: IssueStatus::Open,
        }
    }

    pub fn update_status(&mut self, status: IssueStatus) {
        self.status = status;
    }

    pub fn get_status(&self) -> &IssueStatus {
        &self.status
    }
}

pub struct IssueTracker {
    issues: Vec<Issue>,
}

impl IssueTracker {
    pub fn new() -> Self {
        IssueTracker { issues: Vec::new() }
    }

    pub fn add_issue(&mut self, issue: Issue) {
        self.issues.push(issue);
    }

    pub fn get_issue(&self, id: u32) -> Option<&Issue> {
        self.issues.iter().find(|issue| issue.id == id)
    }

    pub fn update_issue_status(&mut self, id: u32, status: IssueStatus) {
        if let Some(issue) = self.issues.iter_mut().find(|issue| issue.id == id) {
            issue.update_status(status);
        }
    }
}