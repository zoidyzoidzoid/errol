// To parse and unparse this JSON data, add this code to your project and do:
//
//    gitlabPushEvent, err := UnmarshalGitlabPushEvent(bytes)
//    bytes, err = gitlabPushEvent.Marshal()

package server

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"strings"
)

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

func handleGitlabEvent(push GitlabPushEvent) {
	for _, commit := range push.Commits {
		_, err := fmt.Printf("%s: %s\n", commit.ID, strings.Split(commit.Message, "\n")[0])
		if err != nil {
			log.Print("Error printing commit data")
		}
	}
	fmt.Println("Gitlab push received!")
}

func loadGitlabEvent(w http.ResponseWriter, eventType string, body []byte) {
	if eventType != EventTypePush {
		fmt.Fprintf(w, "GitHub event received, but unknown type: %s", eventType)
		return
	}
	push, err := UnmarshalGitlabPushEvent(body)
	if err != nil {
		_, writeErr := fmt.Fprintf(w, "Error reading event.")
		if writeErr != nil {
			log.Print("Error writing response on unmarshalling event")
		}
		return
	}
	handleGitlabEvent(push)
	fmt.Fprintf(w, "Gitlab push received!")
}

type GitlabHandler struct {
	token string
}

func (f GitlabHandler) ServeHTTP(w http.ResponseWriter, req *http.Request) {
	var err error
	if req.Header[GitlabEventHeader] == nil || req.Header[GitlabEventHeader][0] != GitlabEventHookPushValue {
		fmt.Printf("Invalid event received on Gitlab endpoint")
		_, err := fmt.Fprintf(w, "Invalid Gitlab push!")
		if err != nil {
			log.Print("Error writing response")
		}
		return
	}
	fmt.Println("GitHub push received!")
	eventType := EventTypePush
	body, err := ioutil.ReadAll(req.Body)
	if err != nil {
		_, writeErr := fmt.Fprintf(w, "Error reading response body.")
		if writeErr != nil {
			log.Print("Error writing response on error reading body")
		}
		return
	}
	loadGitlabEvent(w, eventType, body)
}
