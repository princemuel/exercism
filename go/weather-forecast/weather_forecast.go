// Package weather provides tools to forecast
// the current weather condition of a particular city.
package weather

// CurrentCondition is a string value equal to the current weather condition of a particular city.
var CurrentCondition string

// CurrentLocation is a string value equal to the location of a particular city.
var CurrentLocation string

// Forecast returns a string value
// the current weather condition in a city and its location.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
