package notifier

import (
	"encoding/json"
	"io/ioutil"

	"gopkg.in/yaml.v2"
)

// UnmarshalJSONRules should have a comment
func UnmarshalJSONRules(data []byte) (Rules, error) {
	var r Rules
	err := json.Unmarshal(data, &r)
	return r, err
}

// UnmarshalYAMLRules should have a comment
func UnmarshalYAMLRules(data []byte) (Rules, error) {
	var r Rules
	err := json.Unmarshal(data, &r)
	return r, err
}

// Marshal should have a comment.
func (r *Rules) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

// Rules contains a list of Rule objects
type Rules struct {
	Rules []Rule `json:"rules"`
}

// Rule describes a single rule that should match either an issue,
// review, or commit.
// Authors just describes someone pushing at the moment.
// Paths described a list of RegEx patterns that will match a file in the commit
// Projects needs to match the repository.full_name in the case of GitHub or
// path_with_namespace for Gitlab.
// To lists the people that should be notified in the case of the rule matching
type Rule struct {
	Name     string   `json:"name"`
	Authors  []string `json:"authors"`
	Paths    []string `json:"paths"`
	Projects []string `json:"projects"`
	ReplyTo  []string `json:"reply_to"`
	To       []string `json:"to"`
}

// LoadRules should have a comment
func LoadRules(path string) (rules Rules, err error) {
	data, err := ioutil.ReadFile(path)
	if err != nil {
		return
	}
	err = yaml.Unmarshal(data, &rules)
	if err != nil {
		return
	}
	return
}
