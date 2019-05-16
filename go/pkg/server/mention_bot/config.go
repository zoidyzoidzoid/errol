package notifier

import "encoding/json"

func UnmarshalMentionBotConfig(data []byte) (MentionBotConfig, error) {
	var r MentionBotConfig
	err := json.Unmarshal(data, &r)
	return r, err
}

func (r *MentionBotConfig) Marshal() ([]byte, error) {
	return json.Marshal(r)
}

type MentionBotConfig struct {
	Actions                []string        `json:"actions"`
	AlwaysNotifyForPaths   []NotifyForPath `json:"alwaysNotifyForPaths"`
	AssignToReviewer       bool            `json:"assignToReviewer"`
	Branches               []string        `json:"branches"`
	CreateComment          bool            `json:"createComment"`
	CreateReviewRequest    bool            `json:"createReviewRequest"`
	Delayed                bool            `json:"delayed"`
	DelayedUntil           string          `json:"delayedUntil"`
	FallbackNotifyForPaths []NotifyForPath `json:"fallbackNotifyForPaths"`
	FileBlacklist          []string        `json:"fileBlacklist"`
	FindPotentialReviewers bool            `json:"findPotentialReviewers"`
	MaxReviewers           int64           `json:"maxReviewers"`
	Message                string          `json:"message"`
	NumFilesToCheck        int64           `json:"numFilesToCheck"`
	RequiredOrgs           []string        `json:"requiredOrgs"`
	SkipAlreadyAssignedPR  bool            `json:"skipAlreadyAssignedPR"`
	SkipAlreadyMentionedPR bool            `json:"skipAlreadyMentionedPR"`
	SkipCollaboratorPR     bool            `json:"skipCollaboratorPR"`
	SkipTitle              string          `json:"skipTitle"`
	UserBlacklist          []string        `json:"userBlacklist"`
	UserBlacklistForPR     []string        `json:"userBlacklistForPR"`
	WithLabel              string          `json:"withLabel"`
}

type NotifyForPath struct {
	Files       []string `json:"files"`
	Name        string   `json:"name"`
	SkipTeamPrs bool     `json:"skipTeamPrs"`
}
