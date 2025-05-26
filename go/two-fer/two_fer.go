// File: two_fer.go
// Package twofer provides functionality to share a cookie with someone.
// It includes a function to format a message indicating who the cookie is for.
// This package is a solution to the Two-fer exercise from the Go Track on Exercism.
package twofer

import (
	"strings"
)

// ShareWith takes a name and returns a string that says who the cookie is for.
// If the name is empty, it defaults to "you".
// The returned string is in the format: "One for <name>, one for me."
func ShareWith(name string) string {
	if strings.TrimSpace(name) == "" {
		name = "you"
	}
	return "One for " + name + ", one for me."
}
