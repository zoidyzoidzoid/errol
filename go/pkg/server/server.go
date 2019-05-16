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
	"log"
	"net/http"

	// pprof for profiling
	"net/http/pprof"
	"time"

	"go.opencensus.io/plugin/ochttp"
	"go.opencensus.io/plugin/ochttp/propagation/b3"
	"go.opencensus.io/zpages"
)

const (
	GitHubProviderName       = "github"
	GitHubEventHeader        = "X-Github-Event"
	GitHubEventHookPushValue = "push"

	GitlabProviderName       = "gitlab"
	GitlabEventHeader        = "X-Gitlab-Event"
	GitlabEventHookPushValue = "Push Hook"

	EventTypePush = "push"
	addr          = ":8082"
)

// var (
// // zpagesAddr = os.Getenv("PROF_HTTP", ":8082", "net/http/pprof http bind address.")
// // profAddr = env.Get("ERROL_PROF_HTTP", ":8082", "net/http/pprof http bind address.")
// 	// profAddr = os.Getenv("ERROL_PROF_HTTP")
// )

// Run should have a comment
func Run() {
	// https://medium.com/observability/debugging-latency-in-go-1-11-9f97a7910d68
	go func() {
		pp := http.NewServeMux()
		pp.Handle("/debug/pprof/", http.HandlerFunc(pprof.Index))
		pp.Handle("/debug/pprof/cmdline", http.HandlerFunc(pprof.Cmdline))
		pp.Handle("/debug/pprof/profile", http.HandlerFunc(pprof.Profile))
		pp.Handle("/debug/pprof/symbol", http.HandlerFunc(pprof.Symbol))
		pp.Handle("/debug/pprof/trace", http.HandlerFunc(pprof.Trace))
		zpages.Handle(pp, "/debug/zpages")
		log.Println("warning: could not start debug HTTP server:", http.ListenAndServe(addr, pp))
	}()

	gitHubHandler := GitHubHandler{}
	gitlabHandler := GitlabHandler{}
	http.Handle("/api/v1/hooks/github", ochttp.WithRouteTag(gitHubHandler, "/api/v1/hooks/github"))
	http.Handle("/api/v1/hooks/gitlab", ochttp.WithRouteTag(gitlabHandler, "/api/v1/hooks/gitlab"))
	srv := &http.Server{
		Addr:         ":8080",
		Handler: &ochttp.Handler{
			Propagation: &b3.HTTPFormat{},
		},
		ReadTimeout:  5 * time.Second,
		WriteTimeout: 10 * time.Second,
	}
	fmt.Printf("Starting server on http://127.0.0.1%s\n", srv.Addr)
	log.Fatal(srv.ListenAndServe())
}
