package blackjack

var cardValues = map[string]int{
	"ace":   11,
	"two":   2,
	"three": 3,
	"four":  4,
	"five":  5,
	"six":   6,
	"seven": 7,
	"eight": 8,
	"nine":  9,
	"ten":   10,
	"jack":  10,
	"queen": 10,
	"king":  10,
}

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {
	if value, exists := cardValues[card]; exists {
		return value
	}
	return 0
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
	if card1 == "ace" && card2 == "ace" {
		return "P"
	}

	c1, c2, dealer := ParseCard(card1), ParseCard(card2), ParseCard(dealerCard)
	playerScore := c1 + c2

	if playerScore == 21 {
		return map[bool]string{true: "S", false: "W"}[dealer >= 10]
	}

	switch {
	case playerScore >= 17:
		return "S"
	case playerScore >= 12:
		return map[bool]string{true: "H", false: "S"}[dealer >= 7]
	default:
		return "H"
	}
}
