// Copyright 2018, OpenCensus Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package server

import (
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"strings"
	"time"
)

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
	if eventType == "push" {

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

func handleGitHubEvent(push GitHubPushEvent) {
	for _, commit := range push.Commits {
		_, err := fmt.Printf("%s: %s\n", commit.ID, strings.Split(commit.Message, "\n")[0])
		if err != nil {
			log.Print("Error printing commit data")
		}
	}
	fmt.Println("Gitlab push received!")
}

func loadGitHubEvent(w http.ResponseWriter, eventType string, body []byte) {
	if eventType != "push" {
		fmt.Fprintf(w, "GitHub event received, but unkown type: %s", eventType)
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

func indexHandler(w http.ResponseWriter, req *http.Request) {
	var err error
	var eventProvider string
	eventType := "push"
	if req.Header["X-Gitlab-Event"] != nil && req.Header["X-Gitlab-Event"][0] == "Push Hook" {
		fmt.Println("Gitlab push received!")
		eventProvider = "Gitlab"
	} else if req.Header["X-GitHub-Event"] != nil && req.Header["X-Gitlab-Event"][0] == "push" {
		fmt.Println("GitHub push received!")
		eventProvider = "GitHub"
	} else {
		_, err := fmt.Fprintf(w, "Invalid Push!")
		if err != nil {
			log.Print("Error writing response")
		}
		return
	}
	body, err := ioutil.ReadAll(req.Body)
	if err != nil {
		_, writeErr := fmt.Fprintf(w, "Error reading response body.")
		if writeErr != nil {
			log.Print("Error writing response on error reading body")
		}
		return
	}
	if eventProvider == "Gitlab" {
		loadGitlabEvent(w, eventType, body)
	} else if eventProvider == "GitHub" {
		loadGitHubEvent(w, eventType, body)
	}
}

// Run should have a comment
func Run() {
	http.HandleFunc("/", indexHandler)
	srv := &http.Server{
		Addr:         ":8080",
		ReadTimeout:  5 * time.Second,
		WriteTimeout: 10 * time.Second,
	}
	fmt.Printf("Starting server on %s\n", srv.Addr)
	log.Fatal(srv.ListenAndServe())
}
