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

// Run should have a comment
func Run() {
	http.HandleFunc("/", func(w http.ResponseWriter, req *http.Request) {
		if req.Header["X-Gitlab-Event"] != nil && req.Header["X-Gitlab-Event"][0] == "Push Hook" {
			fmt.Println("Push received!")
		} else {
			_, err := fmt.Fprintf(w, "Invalid Push!")
			if err != nil {
				log.Print("Error writing response")
			}
			return
		}
		body, err := ioutil.ReadAll(req.Body)
		if err != nil {
			_, err := fmt.Fprintf(w, "Error reading response body.")
			if err != nil {
				log.Print("Error writing response on error reading body")
			}
			return
		}
		push, err := UnmarshalGitlabPushEvent(body)
		if err != nil {
			_, err := fmt.Fprintf(w, "Error reading event.")
			if err != nil {
				log.Print("Error writing response on unmarshalling event")
			}
			return
		}
		for _, commit := range push.Commits {
			_, err := fmt.Printf("%s: %s\n", commit.ID, strings.Split(commit.Message, "\n")[0])
			if err != nil {
				log.Print("Error printing commit data")
			}
		}
		fmt.Fprintf(w, "Push received!")
	})
	srv := &http.Server{
		Addr:         ":8080",
		ReadTimeout:  5 * time.Second,
		WriteTimeout: 10 * time.Second,
	}
	fmt.Printf("Starting server on %s\n", srv.Addr)
	log.Fatal(srv.ListenAndServe())
}
