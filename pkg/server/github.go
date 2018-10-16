// To parse and unparse this JSON data, add this code to your project and do:
//
//    gitHubPushEvent, err := UnmarshalGitHubPushEvent(bytes)
//    bytes, err = gitHubPushEvent.Marshal()

package server

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"strings"
)

func UnmarshalGitHubPushEvent(data []byte) (GitHubPushEvent, error) {
	var r GitHubPushEvent
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *GitHubPushEvent) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type GitHubPushEvent struct {
	Ref        string           `json:"ref"`
	Before     string           `json:"before"`
	After      string           `json:"after"`
	Created    bool             `json:"created"`
	Deleted    bool             `json:"deleted"`
	Forced     bool             `json:"forced"`
	BaseRef    interface{}      `json:"base_ref"`
	Compare    string           `json:"compare"`
	Commits    []GitHubCommit   `json:"commits"`
	HeadCommit GitHubCommit     `json:"head_commit"`
	Repository GitHubRepository `json:"repository"`
	Pusher     GitHubPusher     `json:"pusher"`
	Sender     GitHubSender     `json:"sender"`
}

type GitHubCommit struct {
	ID        string       `json:"id"`
	TreeID    string       `json:"tree_id"`
	Distinct  bool         `json:"distinct"`
	Message   string       `json:"message"`
	Timestamp string       `json:"timestamp"`
	URL       string       `json:"url"`
	Author    GitHubAuthor `json:"author"`
	Committer GitHubAuthor `json:"committer"`
	Added     []string     `json:"added"`
	Removed   []string     `json:"removed"`
	Modified  []string     `json:"modified"`
}

type GitHubAuthor struct {
	Name     string `json:"name"`
	Email    string `json:"email"`
	Username string `json:"username"`
}

type GitHubPusher struct {
	Name  string `json:"name"`
	Email string `json:"email"`
}

type GitHubRepository struct {
	ID               int64        `json:"id"`
	NodeID           string       `json:"node_id"`
	Name             string       `json:"name"`
	FullName         string       `json:"full_name"`
	Private          bool         `json:"private"`
	Owner            GitHubSender `json:"owner"`
	HTMLURL          string       `json:"html_url"`
	Description      interface{}  `json:"description"`
	Fork             bool         `json:"fork"`
	URL              string       `json:"url"`
	ForksURL         string       `json:"forks_url"`
	KeysURL          string       `json:"keys_url"`
	CollaboratorsURL string       `json:"collaborators_url"`
	TeamsURL         string       `json:"teams_url"`
	HooksURL         string       `json:"hooks_url"`
	IssueEventsURL   string       `json:"issue_events_url"`
	EventsURL        string       `json:"events_url"`
	AssigneesURL     string       `json:"assignees_url"`
	BranchesURL      string       `json:"branches_url"`
	TagsURL          string       `json:"tags_url"`
	BlobsURL         string       `json:"blobs_url"`
	GitTagsURL       string       `json:"git_tags_url"`
	GitRefsURL       string       `json:"git_refs_url"`
	TreesURL         string       `json:"trees_url"`
	StatusesURL      string       `json:"statuses_url"`
	LanguagesURL     string       `json:"languages_url"`
	StargazersURL    string       `json:"stargazers_url"`
	ContributorsURL  string       `json:"contributors_url"`
	SubscribersURL   string       `json:"subscribers_url"`
	SubscriptionURL  string       `json:"subscription_url"`
	CommitsURL       string       `json:"commits_url"`
	GitCommitsURL    string       `json:"git_commits_url"`
	CommentsURL      string       `json:"comments_url"`
	IssueCommentURL  string       `json:"issue_comment_url"`
	ContentsURL      string       `json:"contents_url"`
	CompareURL       string       `json:"compare_url"`
	MergesURL        string       `json:"merges_url"`
	ArchiveURL       string       `json:"archive_url"`
	DownloadsURL     string       `json:"downloads_url"`
	IssuesURL        string       `json:"issues_url"`
	PullsURL         string       `json:"pulls_url"`
	MilestonesURL    string       `json:"milestones_url"`
	NotificationsURL string       `json:"notifications_url"`
	LabelsURL        string       `json:"labels_url"`
	ReleasesURL      string       `json:"releases_url"`
	DeploymentsURL   string       `json:"deployments_url"`
	CreatedAt        int64        `json:"created_at"`
	UpdatedAt        string       `json:"updated_at"`
	PushedAt         int64        `json:"pushed_at"`
	GitURL           string       `json:"git_url"`
	SSHURL           string       `json:"ssh_url"`
	CloneURL         string       `json:"clone_url"`
	SvnURL           string       `json:"svn_url"`
	Homepage         string       `json:"homepage"`
	Size             int64        `json:"size"`
	StargazersCount  int64        `json:"stargazers_count"`
	WatchersCount    int64        `json:"watchers_count"`
	Language         string       `json:"language"`
	HasIssues        bool         `json:"has_issues"`
	HasProjects      bool         `json:"has_projects"`
	HasDownloads     bool         `json:"has_downloads"`
	HasWiki          bool         `json:"has_wiki"`
	HasPages         bool         `json:"has_pages"`
	ForksCount       int64        `json:"forks_count"`
	MirrorURL        interface{}  `json:"mirror_url"`
	Archived         bool         `json:"archived"`
	OpenIssuesCount  int64        `json:"open_issues_count"`
	License          interface{}  `json:"license"`
	Forks            int64        `json:"forks"`
	OpenIssues       int64        `json:"open_issues"`
	Watchers         int64        `json:"watchers"`
	DefaultBranch    string       `json:"default_branch"`
	Stargazers       int64        `json:"stargazers"`
	MasterBranch     string       `json:"master_branch"`
}

type GitHubSender struct {
	Name              *string `json:"name,omitempty"`
	Email             *string `json:"email,omitempty"`
	Login             string  `json:"login"`
	ID                int64   `json:"id"`
	NodeID            string  `json:"node_id"`
	AvatarURL         string  `json:"avatar_url"`
	GravatarID        string  `json:"gravatar_id"`
	URL               string  `json:"url"`
	HTMLURL           string  `json:"html_url"`
	FollowersURL      string  `json:"followers_url"`
	FollowingURL      string  `json:"following_url"`
	GistsURL          string  `json:"gists_url"`
	StarredURL        string  `json:"starred_url"`
	SubscriptionsURL  string  `json:"subscriptions_url"`
	OrganizationsURL  string  `json:"organizations_url"`
	ReposURL          string  `json:"repos_url"`
	EventsURL         string  `json:"events_url"`
	ReceivedEventsURL string  `json:"received_events_url"`
	Type              string  `json:"type"`
	SiteAdmin         bool    `json:"site_admin"`
}

func handleGitHubEvent(push GitHubPushEvent) {
	fmt.Printf("Received push: %s %s...%s", push.BaseRef, push.Before, push.After)
	for _, commit := range push.Commits {
		_, err := fmt.Printf("%s: %s\n", commit.ID, strings.Split(commit.Message, "\n")[0])
		if err != nil {
			log.Print("Error printing commit data")
		}
	}
	fmt.Println("Gitlab push received!")
}

func loadGitHubEvent(w http.ResponseWriter, eventType string, body []byte) {
	if eventType != EventTypePush {
		fmt.Fprintf(w, "GitHub event received, but unknown type: %s", eventType)
		return
	}
	push, err := UnmarshalGitHubPushEvent(body)
	if err != nil {
		_, writeErr := fmt.Fprintf(w, "Error reading event.")
		if writeErr != nil {
			log.Print("Error writing response on unmarshalling event")
		}
		return
	}
	handleGitHubEvent(push)
	fmt.Fprintf(w, "GitHub push received!")
}

type GitHubHandler struct {
	token string
}

func (f GitHubHandler) ServeHTTP(w http.ResponseWriter, req *http.Request) {
	var err error
	fmt.Printf("%+v", req.Header)
	if req.Header[GitHubEventHeader] == nil || req.Header[GitHubEventHeader][0] != GitHubEventHookPushValue {
		fmt.Printf("Invalid event received on GitHub endpoint")
		_, err = fmt.Fprintf(w, "Invalid GitHub push")
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
	loadGitHubEvent(w, eventType, body)
}
