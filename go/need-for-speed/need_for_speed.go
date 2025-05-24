package speed

type Car struct {
	battery      int
	speed        int
	batteryDrain int
	distance     int
}

// NewCar creates a new remote controlled car with full battery and given specifications.
func NewCar(speed, batteryDrain int) Car {
	return Car{speed: speed, batteryDrain: batteryDrain, battery: 100, distance: 0}
}

type Track struct {
	distance int
}

// NewTrack creates a new track
func NewTrack(distance int) Track {
	return Track{distance: distance}
}

// Drive drives the car one time. If there is not enough battery to drive one more time,
// the car will not move.
func Drive(car Car) Car {
	batteryLevel := car.battery - car.batteryDrain
	if batteryLevel < 0 {
		return car
	}

	return Car{
		battery:      batteryLevel,
		speed:        car.speed,
		batteryDrain: car.batteryDrain,
		distance:     car.distance + car.speed,
	}
}

// CanFinish checks if a car is able to finish a certain track.
func CanFinish(car Car, track Track) bool {
	distance := car.battery / car.batteryDrain * car.speed
	return distance >= track.distance
}
