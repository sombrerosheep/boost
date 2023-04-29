## Story
Earth is done for. Every country has launched their nukes at each other and its only a matter of time. Luckily, we prepared for this and have survivor rockets on their way to the United Nations moon-base where we can try to rebuild society. Unfortunately, every other country has done the same and the moon can only accept a limited number of survivors.

Right as we're launching, GPS goes offline. It seems each country has targeted each others GPS satellites so their own can reach the moon and take control of the base.

It is now up to you to navigate the rocket manually to the moon! To succeed you must avoid the navigated (and un-navigated) rockets of others, space junk, satellites,  nukes, space rocks and whatever else there might be!

## Game Info
- top-down
- vertical scroll
- defensive; no guns, no bombs, no lasers (ok, maybe lasers)

## World

The game exists from Earth to the moon.
- [ ] Is this only to the moon? #question 
	- maybe there's a series of bad luck and we have to keep jumping from one likely location to the next?
- [ ] How wide is the world? #question 
- [ ] If the mission is to get to the moon, do we skip earths atmosphere? #question
	- leaving the atmosphere is a big "breakthrough" from a layer of condensed junk/satellite?
	- if this starts the level then we wont want it to impact players health
		- One time automatic "burst/boost" that is automatic
		- Can the player elect not to use this one time boost and reserve it for another part?
		- This would start with a disadvantage but could add play diversity

## Mechanics

Boost is a top-down defensive scroller. You must always be moving and will have no "weapons". Your ship has a few features that will help you defend yourself.

You will have Fuel that can run out.
You will want to avoid damage but taking some will be necessary.

### Movement

The game scrolls naturally at a specified pace. Player's control the ship in the play area as it scrolls.

- [ ] Does idle movement go with scroll or will the player fall? #question
    - If the player falls, do they fall off and die?
    - Do they reach a minimum point and then scroll with?

### Fuel

Fuel is a limited resource with a fixed amount. Movement costs fuel. Some abilities costs fuel. 

- [ ] Win the game/round/exit-atmosphere/whatever without any fuel #achievement
    - meaning you spent all your fuel and then remaining velocity pushed to to the goal/end

### Defending yourself

This is a defensive game; the player will have no means to attach or destroy obstacles. Rather the player will need to maneuver around all dangers. The ship will have a few unique features to help it succeed but these will be limited

#### Deflection

This will be the primary way to avoid things; applying force in a direction to alter nearby objects trajectories. Object can and will collide with each other which is part of the strategy.

From weakest to strongest:
- jettison fuel
- Waste-water blast
    - releases waste-water in a specific direction
    - a weak blast for small rocks and light-weight debris
- Vernier Burst
    - Uses vernier thrusters to unleash a "shockwave" in all directions
    - strong blast able to influence heavier objects

- [ ] Can the player choose their own rocket? #idea 
    - Do real rockets have different booster configs to help this make sense?
    - Example: Different rockets have different configurations to alter play-style or challenge?
        - Different vernier booster configs
        - Different fuel capacity
        - Different speeds
        - Ability Cool-down

### Obstacles

#### Rocks
Rocks, They're everywhere!

#### Rockets
These are other people, going for the same goal as you

#### Satellites
- Operational: serving some purpose
- Broken: just floatin'. just junkin'
- Pieces of satellites

- [ ] Should pieces of satellites be a form of space junk instead? #question 
- [ ] Can destroying operational satellites have an impact? #question
    - Either in your favor or not?
    - are they a "question-block" (randomized)

#### Space Junk
- [ ] What does current spacejunk consist of? #question

#### Nukes
- Big and heavy
- Slow moving
- Hit one and you die.
- Push rocks into it and i'll explode and clear out debris for a large area #incentive

#### Placement/Distribution

- [ ] How to control and handle placement of obstacles? #question
- [ ] Does a path need to exist? #question 
    - How to solve/plan for a path to the goal?
    - It has to be winnable.