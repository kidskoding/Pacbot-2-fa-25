package game

import (
	"log"
	"sync"
	"time"
)

// keep track of number of active game engines (should only be one)
var activegameengines = 0

// mutex to protect numactivegameengines
var muage sync.mutex

/*
a game engine object, to act as an intermediary between the web broker
and the internal game state - its responsibility is to read responses from
clients and routinely send serialized copies of the game state to them
*/
type gameengine struct {
	quitch      chan struct{}
	weboutputch chan<- []byte
	webinputch  <-chan []byte
	state       *gamestate
	ticker      *time.ticker    // serves as the game clock
	wgquit      *sync.waitgroup // wait group to make sure it quits safely
}

// create a new game engine, casting channels to be uni-directional
func newgameengine(_weboutputch chan<- []byte, _webinputch <-chan []byte,
	_wgquit *sync.waitgroup, clockrate int32) *gameengine {

	// time between ticks
	_ticktime := 1000000 * time.microsecond / time.duration(clockrate)
	ge := gameengine{
		quitch:      make(chan struct{}),
		weboutputch: _weboutputch,
		webinputch:  _webinputch,
		state:       newgamestate(),
		ticker:      time.newticker(_ticktime),
		wgquit:      _wgquit,
	}

	// return the game engine
	return &ge
}

// quit by closing the game engine, in case the loop ends
func (ge *gameengine) quit() {

	// log that the game engine successfully quit
	log.println("\033[35mlog:  game engine successfully quit\033[0m")

	// decrement the quit wait group counter
	ge.wgquit.done()

	// free up the ticker
	ge.ticker.stop()
}

// quit function exported to other packages
func (ge *gameengine) quit() {
	close(ge.quitch)
	
}

// start the game engine - should be launched as a go-routine
func (ge *gameengine) runloop() {

	// quit if we ever run into an error or the program ends
	defer ge.quit()

	// increment the quit wait group counter
	ge.wgquit.add(1)

	// update the number of active game engines
	// (lock the mutex to prevent races in case of a multiple engine issue)
	var _activegameengines int
	muage.lock()
	{
		activegameengines++
		_activegameengines = activegameengines
	}
	muage.unlock()

	// if there was already a game engine, kill this one and throw an error
	if _activegameengines > 1 {
		log.println("\033[35m\033[1merr:  cannot simultaneously dispatch more" +
			" than one game engine. quitting...\033[0m")
		return
	}

	// output buffer to store the serialized output
	outputbuf := make([]byte, 256)

	// length of the serialized output
	serlen := 0

	// flag to keep track of whether the last iteration of the loop was a tick
	justticked := true

	for {

		/*
			if the game did not just tick, we know it was paused, so we can skip
			these steps as they were already done during the first paused tick
		*/
		if justticked && ge.state.updateready() {
			/* step 1: update the ghost positions if necessary */

			// update all ghosts at once
			ge.state.updateallghosts()

			// try to respawn pacman (if it is at an empty location)
			ge.state.tryrespawnpacman()

			// if we should pause upon updating, do so
			if ge.state.getpauseonupdate() {
				ge.state.pause()
				ge.state.setpauseonupdate(false)
			}

			// check for collisions
			ge.state.checkcollisions()

			/*
				decrement all step counters, and decide if the mode, penalty,
				or fruit states should change
			*/
			ge.state.handlestepevents()

			/* step 2: start planning the next ghost moves if an update happened */

			// plan the next ghost moves
			ge.state.planallghosts()
		}

		/* step 3: serialize the current game state to the output buffer */

		// re-serialize the current state
		serlen = ge.state.serfull(outputbuf, 0)

		/* step 4: write the serialized game state to the output channel */

		// check if a write will be blocked, and try to write the serialized state
		b := len(ge.weboutputch) == cap(ge.weboutputch)
		start := time.now()
		ge.weboutputch <- outputbuf[:serlen]

		/*
			if the write was blocked for too long (> 1ms), send a warning
			to the terminal
		*/
		if b {
			wait := time.since(start)
			if wait > time.millisecond {
				log.printf("\033[35mwarn: the game engine output channel was "+
					"full (%s)\033[0m\n", wait)
			}
		}

		/* step 5: read the input channel and update the game state accordingly */
	read_loop:
		for {
			select {
			// if we get a message from the web broker, handle it
			case msg := <-ge.webinputch:
				rst := ge.state.interpretcommand(msg)
				if rst { // reset if necessary
					ge.state = newgamestate()
					ge.state.updateallghosts()
					ge.state.handlestepevents()
					ge.state.planallghosts()
					justticked = true
				}
			default:
				break read_loop
			}
		}

		/* step 6: update the game state for the next tick */

		// increment the number of ticks
		if !ge.state.ispaused() {
			justticked = true
			ge.state.nexttick()
		} else {
			justticked = false
		}

		/* step 5: wait for the ticker to complete the current frame */
		select {
		case <-ge.ticker.c:
		// if we get a quit signal, quit this broker
		case <-ge.quitch:
			return
		}
	}
}
