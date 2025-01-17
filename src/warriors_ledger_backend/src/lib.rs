#[macro_use]
extern crate serde;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

mod types;
use types::*;

//Declare thread local variables
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static PLAYER_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );
    static WEAPON_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 0)
            .expect("Cannot create a counter")
    );
    static MATCH_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), 0)
            .expect("Cannot create a counter")
    );
    static LEADERBOARD_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))), 0)
            .expect("Cannot create a counter")
    );
    static PLAYER_PROFILE_STORAGE: RefCell<StableBTreeMap<u64, PlayerProfile, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );
    static WEAPON_PROFILE_STORAGE: RefCell<StableBTreeMap<u64, Weapon, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );
    static MATCH_PROFILE_STORAGE: RefCell<StableBTreeMap<u64, Match, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6))))
    );
    static LEADERBOARD_STORAGE: RefCell<StableBTreeMap<u64, Leaderboard, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7))))
    );


}

//function to create player profile
#[ic_cdk::update]
fn create_player_profile(
    player_profile_payload: PlayerProfilePayload,
) -> Result<PlayerProfile, Error> {

    is_valid_player_payload(&player_profile_payload)?;
    let id = PLAYER_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let player_profile = PlayerProfile {
        name: player_profile_payload.name,
        id,
        score: player_profile_payload.score,
        level: player_profile_payload.level,
        rank: player_profile_payload.rank,
        weapons: Vec::new(),
        match_history: Vec::new(),
    };
    do_insert_player(&player_profile);
    Ok(player_profile)

}


// helper function to get player profile
fn do_insert_player(player: &PlayerProfile) {
    PLAYER_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(player.id, player.clone())
    });
}

//function to update player profile
#[ic_cdk::update]
fn update_player_profile(id:u64,player_profile_payload: PlayerProfilePayload)-> Result<PlayerProfile, Error>{
    is_valid_player_payload(&player_profile_payload)?;
    let player_profile = PLAYER_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: "Player not found".to_string(),
            })
    })?;

    let updated_player_profile = PlayerProfile {
        name: player_profile_payload.name,
        id,
        score: player_profile_payload.score,
        level: player_profile_payload.level,
        rank: player_profile_payload.rank,
        weapons: player_profile.weapons,
        match_history: player_profile.match_history,
    };

    do_insert_player(&updated_player_profile);
    Ok(updated_player_profile)

}


// get player profile by id
#[ic_cdk::query]
fn get_player_profile(id: u64) -> Result<PlayerProfile, Error> {
    PLAYER_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("player with id={} not found", id),
            })
    })
}

// function to get all players profile
#[ic_cdk::query]
fn get_all_players_profile() -> Result<Vec<PlayerProfile>, Error> {
    let player_mapping: Vec<(u64, PlayerProfile)> =
        PLAYER_PROFILE_STORAGE.with(|service| service.borrow().iter().collect());
    let player_profile: Vec<PlayerProfile> = player_mapping
        .into_iter()
        .map(|(_, player)| player)
        .collect();

    if !player_profile.is_empty() {
        Ok(player_profile)
    } else {
        Err(Error::NotFound {
            msg: "No players found ".to_string(),
        })
    }
}

//function to delete player profile
#[ic_cdk::update]
fn delete_player_profile(id: u64) -> Result<(), Error> {
    PLAYER_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .remove(&id)
            .ok_or(Error::NotFound {
                msg: format!("player with id={} not found", id),
            })
    })?;
    Ok(())
}

//function to create weapon profile
#[ic_cdk::update]
fn create_weapon(
    weapon_payload: WeaponProfilePayload,
) -> Result<Weapon, Error> {
    is_valid_weapon_payload(&weapon_payload)?;

    let id = WEAPON_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let weapon = Weapon {
        name: weapon_payload.name,
        id,
        damage: weapon_payload.damage,
        ammo: weapon_payload.ammo,
        range: weapon_payload.range,
        fire_rate: weapon_payload.fire_rate,
        reload_time: weapon_payload.reload_time,
        accuracy: weapon_payload.accuracy,
        price: weapon_payload.price,
        level: weapon_payload.level,
        rank: weapon_payload.rank,
    };
    do_insert_weapon(&weapon);
    Ok(weapon)
}

// helper function to get weapon profile
fn do_insert_weapon(weapon: &Weapon) {
    WEAPON_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(weapon.id, weapon.clone())
    });
}

//function to update weapon profile
#[ic_cdk::update]
fn update_weapon_profile(id:u64,weapon_payload: WeaponProfilePayload)-> Result<Weapon, Error>{
    is_valid_weapon_payload(&weapon_payload)?;

    WEAPON_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: "Weapon not found".to_string(),
            })
    })?;

    let updated_weapon = Weapon {
        name: weapon_payload.name,
        id,
        damage: weapon_payload.damage,
        ammo: weapon_payload.ammo,
        range: weapon_payload.range,
        fire_rate: weapon_payload.fire_rate,
        reload_time: weapon_payload.reload_time,
        accuracy: weapon_payload.accuracy,
        price: weapon_payload.price,
        level: weapon_payload.level,
        rank: weapon_payload.rank,
    };

    do_insert_weapon(&updated_weapon);
    Ok(updated_weapon)


}


// get weapon by id
#[ic_cdk::query]
fn get_weapon(id: u64) -> Result<Weapon, Error> {
    WEAPON_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("weapon with id={} not found", id),
            })
    })
}

// function to get all weapons
#[ic_cdk::query]
fn get_all_weapons() -> Result<Vec<Weapon>, Error> {
    let weapon_mapping: Vec<(u64, Weapon)> =
        WEAPON_PROFILE_STORAGE.with(|service| service.borrow().iter().collect());
    let weapon: Vec<Weapon> = weapon_mapping
        .into_iter()
        .map(|(_, weapon)| weapon)
        .collect();

    if !weapon.is_empty() {
        Ok(weapon)
    } else {
        Err(Error::NotFound {
            msg: "No weapons found ".to_string(),
        })
    }
}
//rank weapons by damage
#[ic_cdk::query]
fn rank_weapons_by_damage() -> Result<Vec<Weapon>, Error> {
    let weapon_mapping: Vec<(u64, Weapon)> =
        WEAPON_PROFILE_STORAGE.with(|service| service.borrow().iter().collect());
    let mut weapon: Vec<Weapon> = weapon_mapping
        .into_iter()
        .map(|(_, weapon)| weapon)
        .collect();

    if !weapon.is_empty() {
        weapon.sort_by(|a, b| b.damage.cmp(&a.damage));
        Ok(weapon)
    } else {
        Err(Error::NotFound {
            msg: "No weapons found ".to_string(),
        })
    }
}
//function to delete weapon profile
#[ic_cdk::update]
fn delete_weapon(id: u64) -> Result<(), Error> {
    WEAPON_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .remove(&id)
            .ok_or(Error::NotFound {
                msg: format!("weapon with id={} not found", id),
            })
    })?;
    do_delete_weapon_from_player_profiles(id);
    Ok(())
}

// remove weapon from player profiles
fn do_delete_weapon_from_player_profiles(weapon_id: u64) {
    let player_mapping: Vec<(u64, PlayerProfile)> =
        PLAYER_PROFILE_STORAGE.with(|service| service.borrow().iter().collect());
    let mut player_profile: Vec<PlayerProfile> = player_mapping
        .into_iter()
        .map(|(_, player)| player)
        .collect();

    for player in player_profile.iter_mut() {
        let mut weapons = player.weapons.clone();
        weapons.retain(|weapon| weapon.id != weapon_id);
        player.weapons = weapons;
        do_insert_player(&player);
    }
}



//add weapon to player profile
#[ic_cdk::update]
fn add_weapon_to_player_profile(player_id: u64, weapon_id: u64) -> Result<(), Error> {
    let player_profile = PLAYER_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&player_id)
            .ok_or(Error::NotFound {
                msg: format!("player with id={} not found", player_id),
            })
    })?;

    let weapon = WEAPON_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&weapon_id)
            .ok_or(Error::NotFound {
                msg: format!("weapon with id={} not found", weapon_id),
            })
    })?;

    let mut player_profile = player_profile.clone();
    player_profile.weapons.push(weapon.clone());
    do_insert_player(&player_profile);
    Ok(())
}

//function to create match
#[ic_cdk::update]
fn create_match(
    match_payload: MatchProfilePayload,
) -> Result<Match, Error> {
    is_valid_match_payload(&match_payload)?;

    let id = MATCH_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let match_profile = Match {
        id,
        player_id: match_payload.player_id,
        weapon_id: match_payload.weapon_id,
        score: match_payload.score,
        level: match_payload.level,
        rank: match_payload.rank,
        time: match_payload.time,
        result: match_payload.result,
    };
    do_insert_match(&match_profile);
    Ok(match_profile)
}

// helper function to get match profile
fn do_insert_match(match_profile: &Match) {
    MATCH_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(match_profile.id, match_profile.clone())
    });
}

//function to update match
#[ic_cdk::update]
fn update_match(id:u64,match_payload: MatchProfilePayload)-> Result<Match, Error>{
    is_valid_match_payload(&match_payload)?;
    MATCH_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: "Match not found".to_string(),
            })
    })?;

    let updated_match = Match {
        id,
        player_id: match_payload.player_id, 
        weapon_id: match_payload.weapon_id,
        score: match_payload.score,
        level: match_payload.level,
        rank: match_payload.rank,
        time: match_payload.time,
        result: match_payload.result,
    };

    do_insert_match(&updated_match);
    Ok(updated_match)   
}

// get match by id
#[ic_cdk::query]
fn get_match(id: u64) -> Result<Match, Error> {
    MATCH_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("match with id={} not found", id),
            })
    })
}

// function to get all matches
#[ic_cdk::query]
fn get_all_matches() -> Result<Vec<Match>, Error> {
    let match_mapping: Vec<(u64, Match)> =
        MATCH_PROFILE_STORAGE.with(|service| service.borrow().iter().collect());
    let match_profile: Vec<Match> = match_mapping
        .into_iter()
        .map(|(_, match_profile)| match_profile)
        .collect();

    if !match_profile.is_empty() {
        Ok(match_profile)
    } else {
        Err(Error::NotFound {
            msg: "No matches found ".to_string(),
        })
    }
}

//function to delete match
#[ic_cdk::update]
fn delete_match(id: u64) -> Result<(), Error> {
    MATCH_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .remove(&id)
            .ok_or(Error::NotFound {
                msg: format!("match with id={} not found", id),
            })
    })?;
    do_delete_match_from_player_profiles(id);
    Ok(())
}

// remove match from player profiles
fn do_delete_match_from_player_profiles(match_id: u64) {
    let player_mapping: Vec<(u64, PlayerProfile)> =
        PLAYER_PROFILE_STORAGE.with(|service| service.borrow().iter().collect());
    let mut player_profile: Vec<PlayerProfile> = player_mapping
        .into_iter()
        .map(|(_, player)| player)
        .collect();

    for player in player_profile.iter_mut() {
        let mut matches = player.match_history.clone();
        matches.retain(|match_profile| match_profile.id != match_id);
        player.match_history = matches;
        do_insert_player(&player);
    }
}


//add match to player profile
#[ic_cdk::update]
fn add_match_to_player_profile(player_id: u64, match_id: u64) -> Result<(), Error> {
    let player_profile = PLAYER_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&player_id)
            .ok_or(Error::NotFound {
                msg: format!("player with id={} not found", player_id),
            })
    })?;

    let match_profile = MATCH_PROFILE_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&match_id)
            .ok_or(Error::NotFound {
                msg: format!("match with id={} not found", match_id),
            })
    })?;

    let mut player_profile = player_profile.clone();
    player_profile.match_history.push(match_profile.clone());
    do_insert_player(&player_profile);
    Ok(())
}

// get average for match score
#[ic_cdk::query]
fn get_average_match_score() -> Result<u64, Error> {
    let match_mapping: Vec<(u64, Match)> =
        MATCH_PROFILE_STORAGE.with(|service| service.borrow().iter().collect());
    let match_profile: Vec<Match> = match_mapping
        .into_iter()
        .map(|(_, match_profile)| match_profile)
        .collect();

    if !match_profile.is_empty() {
        let mut sum = 0;
        for match_profile in match_profile.iter() {
            sum += match_profile.score;
        }
        let average = sum / match_profile.len() as u64;
        Ok(average)
    } else {
        Err(Error::NotFound {
            msg: "No matches found ".to_string(),
        })
    }
}

// leaderboard

//function to create leaderboard
#[ic_cdk::update]
fn create_leaderboard(
    leaderboard_payload: LeaderboardPayload,
) -> Result<Leaderboard, Error> {
    is_valid_leaderboard_payload(&leaderboard_payload)?;

    let id = LEADERBOARD_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let leaderboard = Leaderboard {
        id,
        player_id: leaderboard_payload.player_id,
        score: leaderboard_payload.score,
        level: leaderboard_payload.level,
        rank: leaderboard_payload.rank,
    };
    do_insert_leaderboard(&leaderboard);
    Ok(leaderboard)
}

// helper function to get leaderboard

fn do_insert_leaderboard(leaderboard: &Leaderboard) {
    LEADERBOARD_STORAGE.with(|service| {
        service
            .borrow_mut()
            .insert(leaderboard.id, leaderboard.clone())
    });
}

//function to update leaderboard
#[ic_cdk::update]
fn update_leaderboard(id:u64,leaderboard_payload: LeaderboardPayload)-> Result<Leaderboard, Error>{
    is_valid_leaderboard_payload(&leaderboard_payload)?;
    LEADERBOARD_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: "Leaderboard not found".to_string(),
            })
    })?;

    let updated_leaderboard = Leaderboard {
        id,
        player_id: leaderboard_payload.player_id,
        score: leaderboard_payload.score,
        level: leaderboard_payload.level,
        rank: leaderboard_payload.rank,
    };

    do_insert_leaderboard(&updated_leaderboard);
    Ok(updated_leaderboard)   
}

// get leaderboard by id
#[ic_cdk::query]
fn get_leaderboard(id: u64) -> Result<Leaderboard, Error> {
    LEADERBOARD_STORAGE.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("leaderboard with id={} not found", id),
            })
    })
}

// function to get all leaderboards for different game played
#[ic_cdk::query]
fn get_all_leaderboards() -> Result<Vec<Leaderboard>, Error> {
    let leaderboard_mapping: Vec<(u64, Leaderboard)> =
        LEADERBOARD_STORAGE.with(|service| service.borrow().iter().collect());
    let leaderboard: Vec<Leaderboard> = leaderboard_mapping
        .into_iter()
        .map(|(_, leaderboard)| leaderboard)
        .collect();

    if !leaderboard.is_empty() {
        Ok(leaderboard)
    } else {
        Err(Error::NotFound {
            msg: "No leaderboards found ".to_string(),
        })
    }
}

//function to delete leaderboard
#[ic_cdk::update]
fn delete_leaderboard(id: u64) -> Result<(), Error> {
    LEADERBOARD_STORAGE.with(|service| {
        service
            .borrow_mut()
            .remove(&id)
            .ok_or(Error::NotFound {
                msg: format!("leaderboard with id={} not found", id),
            })
    })?;
    Ok(())
}

// sort leaderboard by score in descending order
#[ic_cdk::query]
fn sort_leaderboard_by_score() -> Result<Vec<Leaderboard>, Error> {
    let leaderboard_mapping: Vec<(u64, Leaderboard)> =
        LEADERBOARD_STORAGE.with(|service| service.borrow().iter().collect());
    let mut leaderboard: Vec<Leaderboard> = leaderboard_mapping
        .into_iter()
        .map(|(_, leaderboard)| leaderboard)
        .collect();

    if !leaderboard.is_empty() {
        leaderboard.sort_by(|a, b| b.score.cmp(&a.score));
        Ok(leaderboard)
    } else {
        Err(Error::NotFound {
            msg: "No leaderboards found ".to_string(),
        })
    }
}

// Helper function to ensure the input payload does not contain default values
fn is_valid_player_payload(player_profile_payload: &PlayerProfilePayload) -> Result<(), Error>{
    if player_profile_payload.name.trim().is_empty()
    || player_profile_payload.score == 0
    || player_profile_payload.level == 0
    || player_profile_payload.rank == 0
{
    return Err(Error::InvalidPlayerPayload {
         msg: format!("Player profile cannot be initialized with default values"),
         payload: player_profile_payload.clone() 
        });
}
else{
    Ok(())
}
}
// Helper function to ensure the input payload does not contain default values
fn is_valid_weapon_payload(weapon_payload: &WeaponProfilePayload) -> Result<(), Error>{
        if weapon_payload.name.trim().is_empty()
        || weapon_payload.damage == 0
        || weapon_payload.ammo == 0
        || weapon_payload.range == 0
        || weapon_payload.fire_rate == 0
        || weapon_payload.reload_time == 0
        || weapon_payload.accuracy == 0
        || weapon_payload.price == 0
        || weapon_payload.level == 0
        || weapon_payload.rank == 0
    {
        return Err(Error::InvalidWeaponPayload {
            msg: format!("Weapon profile cannot be initialized with default values"),
            payload: weapon_payload.clone() 
        });
    }
else{
    Ok(())
}
}
// Helper function to ensure the input payload does not contain default values
fn is_valid_match_payload(match_payload: &MatchProfilePayload) -> Result<(), Error>{
    if match_payload.score == 0
    || match_payload.level == 0
    || match_payload.rank == 0
    || match_payload.time == 0
{
    return Err(Error::InvalidMatchPayload {
        msg: format!("Match cannot be initialized with default values"),
        payload: match_payload.clone() 
    });
}
let is_valid_player_id = PLAYER_PROFILE_STORAGE.with(|service| {
    service
        .borrow()
        .contains_key(&match_payload.player_id)
});
if !is_valid_player_id {
    return Err(Error::NotFound { msg: format!("Player with id={} does not exist.", match_payload.player_id) })
}
else{
Ok(())
}
}
// Helper function to ensure the input payload does not contain default values
fn is_valid_leaderboard_payload(leaderboard_payload: &LeaderboardPayload) -> Result<(), Error>{
    if  leaderboard_payload.score == 0
    || leaderboard_payload.level == 0
    || leaderboard_payload.rank == 0
{
    return Err(Error::InvalidLeaderboardPayload {
        msg: format!("Leaderboard cannot be initialized with default values"),
        payload: leaderboard_payload.clone() 
    });
}
let is_valid_player_id = PLAYER_PROFILE_STORAGE.with(|service| {
    service
        .borrow()
        .contains_key(&leaderboard_payload.player_id)
});
if !is_valid_player_id {
    return Err(Error::NotFound { msg: format!("Player with id={} does not exist.", leaderboard_payload.player_id) })
}
else{
Ok(())
}
}


// Export the candid interface
ic_cdk::export_candid!();