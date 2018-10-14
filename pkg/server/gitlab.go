// To parse and unparse this JSON data, add this code to your project and do:
//
//    gitlabPushEvent, err := UnmarshalGitlabPushEvent(bytes)
//    bytes, err = gitlabPushEvent.Marshal()

package server

import "encoding/json"

// UnmarshalGitlabPushEvent should have a comment
func UnmarshalGitlabPushEvent(data []byte) (GitlabPushEvent, error) {
	var r GitlabPushEvent
	err := json.Unmarshal(data, &r)
	return r, err
}

// Marshal should have a comment
func (r *GitlabPushEvent) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

// GitlabPushEvent should have a comment
type GitlabPushEvent struct {
	ObjectKind        string           `json:"object_kind"`
	Before            string           `json:"before"`
	After             string           `json:"after"`
	Ref               string           `json:"ref"`
	CheckoutSHA       string           `json:"checkout_sha"`
	UserID            int64            `json:"user_id"`
	UserName          string           `json:"user_name"`
	UserUsername      string           `json:"user_username"`
	UserEmail         string           `json:"user_email"`
	UserAvatar        string           `json:"user_avatar"`
	ProjectID         int64            `json:"project_id"`
	Project           Project          `json:"project"`
	Repository        GitlabRepository `json:"repository"`
	Commits           []Commit         `json:"commits"`
	TotalCommitsCount int64            `json:"total_commits_count"`
}

// Commit should have a comment
type Commit struct {
	ID        string        `json:"id"`
	Message   string        `json:"message"`
	Timestamp string        `json:"timestamp"`
	URL       string        `json:"url"`
	Author    Author        `json:"author"`
	Added     []string      `json:"added"`
	Modified  []string      `json:"modified"`
	Removed   []interface{} `json:"removed"`
}

// Author should have a comment
type Author struct {
	Name  string `json:"name"`
	Email string `json:"email"`
}

// Project should have a comment
type Project struct {
	ID                int64       `json:"id"`
	Name              string      `json:"name"`
	Description       string      `json:"description"`
	WebURL            string      `json:"web_url"`
	AvatarURL         interface{} `json:"avatar_url"`
	GitSSHURL         string      `json:"git_ssh_url"`
	GitHTTPURL        string      `json:"git_http_url"`
	Namespace         string      `json:"namespace"`
	VisibilityLevel   int64       `json:"visibility_level"`
	PathWithNamespace string      `json:"path_with_namespace"`
	DefaultBranch     string      `json:"default_branch"`
	Homepage          string      `json:"homepage"`
	URL               string      `json:"url"`
	SSHURL            string      `json:"ssh_url"`
	HTTPURL           string      `json:"http_url"`
}

// GitlabRepository should have a comment
type GitlabRepository struct {
	Name            string `json:"name"`
	URL             string `json:"url"`
	Description     string `json:"description"`
	Homepage        string `json:"homepage"`
	GitHTTPURL      string `json:"git_http_url"`
	GitSSHURL       string `json:"git_ssh_url"`
	VisibilityLevel int64  `json:"visibility_level"`
}
