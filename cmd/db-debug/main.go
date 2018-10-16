// Code partially from https://raw.githubusercontent.com/src-d/go-git/master/_examples/showcase/main.go
package main

import (
	"context"
	"fmt"
	"os"
	"strings"

	"github.com/mongodb/mongo-go-driver/mongo"
)

// https://godoc.org/github.com/mongodb/mongo-go-driver/mongo

// CheckArgs should be used to ensure the right command line arguments are
// passed before executing an example.
func CheckArgs(arg ...string) {
	if len(os.Args) < len(arg)+1 {
		Warning("Usage: %s %s", os.Args[0], strings.Join(arg, " "))
		os.Exit(1)
	}
}

// CheckIfError should be used to naively panics if an error is not nil.
func CheckIfError(err error) {
	if err == nil {
		return
	}

	fmt.Printf("\x1b[31;1m%s\x1b[0m\n", fmt.Sprintf("error: %s", err))
	os.Exit(1)
}

// Info should be used to describe the example commands that are about to run.
func Info(format string, args ...interface{}) {
	fmt.Printf("\x1b[34;1m%s\x1b[0m\n", fmt.Sprintf(format, args...))
}

// Warning should be used to display a warning
func Warning(format string, args ...interface{}) {
	fmt.Printf("\x1b[36;1m%s\x1b[0m\n", fmt.Sprintf(format, args...))
}

// Open an existing repository in a specific folder.
func main() {
	CheckArgs("<path>")
	path := os.Args[1]
	fmt.Println("Received arg:", path)

	server_addr := os.Getenv("MONGODB_SERVER_URL")
	if server_addr == "" {
		server_addr = "127.0.0.1:27017"
	}
	client, err := mongo.NewClient(fmt.Sprintf("mongodb://%s", server_addr))
	CheckIfError(err)
	err = client.Connect(context.TODO())
	CheckIfError(err)
	collection := client.Database("errol").Collection("github_events")
	fmt.Println(collection)
}
