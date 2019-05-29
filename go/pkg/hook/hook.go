package hook

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"

	"gopkg.in/src-d/go-git.v4"
	"gopkg.in/src-d/go-git.v4/plumbing"
	"gopkg.in/src-d/go-git.v4/plumbing/object"
	"gopkg.in/src-d/go-git.v4/plumbing/storer"
)

// store h in set, s, handling nil s if necessary. Return new set.
func store(s map[plumbing.Hash]bool, h plumbing.Hash) map[plumbing.Hash]bool {
	if s == nil {
		s = make(map[plumbing.Hash]bool)
	}
	s[h] = true
	return s
}

// mergeBase finds best common ancestors between two commits to use in a
// three-way merge. One common ancestor is better than another common ancestor
// if the latter is an ancestor of the former. A common ancestor that does not
// have any better common ancestor is a best common ancestor, i.e. a merge base.
// Note that there can be more than one merge base for a pair of commits.
func mergeBase(s storer.EncodedObjectStorer, a, b plumbing.Hash) ([]plumbing.Hash, error) {
	commitA, err := object.GetCommit(s, a)
	if err != nil {
		return nil, err
	}

	commitB, err := object.GetCommit(s, b)
	if err != nil {
		return nil, err
	}

	// Mapping of direct descendants of each commit we visit
	desc := make(map[plumbing.Hash]map[plumbing.Hash]bool)

	// Set of commits reachable from a
	reachableFromA := make(map[plumbing.Hash]bool)

	// Walk commits reachable from A
	err = object.NewCommitPreorderIter(commitA, nil, nil).ForEach(func(c *object.Commit) error {
		reachableFromA[c.Hash] = true
		for _, h := range c.ParentHashes {
			desc[h] = store(desc[h], c.Hash)
		}
		return nil
	})
	if err != nil {
		return nil, err
	}

	// Set of common commits between a and b
	common := make(map[plumbing.Hash]bool)

	// Walk commits reachable from B
	err = object.NewCommitPreorderIter(commitB, nil, nil).ForEach(func(c *object.Commit) error {
		if reachableFromA[c.Hash] {
			common[c.Hash] = true
		}
		for _, h := range c.ParentHashes {
			desc[h] = store(desc[h], c.Hash)
		}
		return nil
	})
	if err != nil {
		return nil, err
	}

	best := make(map[plumbing.Hash]bool)

	// Trim down the set of common commits to only those that are best
	for h := range common {
		best[h] = true
		for child := range desc[h] {
			if common[child] {
				// there is a descendant to h that is common to both a and b. h is not in best.
				delete(best, h)
				break
			}
		}
	}

	var result []plumbing.Hash
	for h := range best {
		result = append(result, h)
	}
	return result, nil
}

// CheckIfError should hav ea comment
func CheckIfError(err error) {
	if err != nil {
		log.Fatal(err)
	}
}

func processRefChange(oldRevision, newRevision, refName string) {
	log.SetFlags(log.LstdFlags | log.Lshortfile)

	fmt.Println(oldRevision, newRevision, refName)
	// We instanciate a new repository targeting the given path (the .git folder)
	cwd, err := os.Getwd()
	cwd = fmt.Sprintf("%s/%s", cwd, "..")
	r, err := git.PlainOpenWithOptions(cwd, &git.PlainOpenOptions{DetectDotGit: true})
	CheckIfError(err)

	fmt.Printf("%+v\n", r)

	// ... retrieving the HEAD reference
	ref, err := r.Head()
	CheckIfError(err)

	// ... retrieves the commit history
	cIter, err := r.Log(&git.LogOptions{From: ref.Hash()})
	CheckIfError(err)

	// ... just iterates over the commits
	var cCount int
	err = cIter.ForEach(func(c *object.Commit) error {
		cCount++
		// fmt.Println(c.Hash, strings.Split(c.Message, "\n")[0])

		return nil
	})
	CheckIfError(err)

	fromHash := plumbing.NewHash(oldRevision)
	toHash := plumbing.NewHash(newRevision)

	fmt.Println(fromHash, toHash)

	bases, err := mergeBase(r.Storer, fromHash, toHash)
	CheckIfError(err)

	for _, b := range bases {
		fmt.Println(b)
	}

	fmt.Println(ref.Hash())

	// fmt.Println(r.CommitObjects())
	// toTree, err := r.TreeObject(toHash)
	// CheckIfError(fmt.Errorf("Failed to get tree for new Hash: %s", err))
	// fromTree, err := r.TreeObject(fromHash)
	// CheckIfError(fmt.Errorf("Failed to get tree for old Hash: %s", err))

	// changes, err := object.DiffTree(fromTree, toTree)
	// CheckIfError(err)
	// fmt.Println(changes)

	// fmt.Println(cCount)
}

// Run should have a comment
func Run() {
	args := os.Args[1:]
	if 3 > len(args) && len(args) > 1 {
		log.Fatal("Wrong number of arguments, expected 3")
	}
	if len(args) == 3 {
		oldRevision := args[0]
		newRevision := args[1]
		refName := args[2]
		processRefChange(oldRevision, newRevision, refName)
	} else {
		scanner := bufio.NewScanner(os.Stdin)
		for scanner.Scan() {
			args = strings.Split(scanner.Text(), " ")
			oldRevision := args[0]
			newRevision := args[1]
			refName := args[2]
			processRefChange(oldRevision, newRevision, refName)
		}

		if scanner.Err() != nil {
			log.Fatal(scanner.Err())
		}
	}
}
