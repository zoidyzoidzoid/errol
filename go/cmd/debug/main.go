// Code partially from https://raw.githubusercontent.com/src-d/go-git/master/_examples/showcase/main.go
package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"strings"

	"github.com/dghubble/trie"
	"github.com/zoidbergwill/errol/go/pkg/server/notifier"
	"gopkg.in/yaml.v2"
)

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
	Info("Reading %s", path)
	data, err := ioutil.ReadFile(path)
	CheckIfError(err)
	// fmt.Print(string(data))
	var rules notifier.Rules
	err = yaml.Unmarshal(data, &rules)
	if err != nil {
		log.Fatalf("cannot unmarshal data: %v", err)
	}
	// fmt.Printf("%+v\n", rules)
	pathsTrie := trie.NewRuneTrie()
	for _, rule := range rules.Rules {
		for _, path := range rule.Paths {
			if value := pathsTrie.Get(path); value != nil {
				pathsTrie.Put(path, append(value.([]string), rule.To...))
			} else {
				pathsTrie.Put(path, rule.To)
			}
		}
	}
	walker := func(key string, value interface{}) error {
		fmt.Printf("%s: %+v\n", key, value)
		return nil
	}
	err = pathsTrie.Walk(walker)
	CheckIfError(err)
}
