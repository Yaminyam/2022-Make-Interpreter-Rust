pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        //unimplemented!("Revive this player")
        if self.health == 0 {
            if self.level >= 10 {
                return Some(Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                });
            } else {
                return Some(Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                });
            }
        } else {
            return None;
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        unimplemented!("Cast a spell of cost {}", mana_cost)
        if self.mana == None {
            if self.health > mana_cost {
                self.health -= mana_cost;
            } else {
                self.health = 0;
            }
            return 0;
        } else if self.mana.unwrap() < mana_cost {
            return 0;
        } else {
            self.mana = Some(self.mana.unwrap() - mana_cost);
            return mana_cost*2;
        }
    }
}

#[test]
fn test_reviving_dead_player() {
    let dead_player = Player {
        health: 0,
        mana: Some(0),
        level: 34,
    };
    let revived_player = dead_player
        .revive()
        .expect("reviving a dead player must return Some(player)");
    assert_eq!(revived_player.health, 100);
    assert_eq!(revived_player.mana, Some(100));
    assert_eq!(revived_player.level, dead_player.level);
}

#[test]
fn test_reviving_dead_level9_player() {
    let dead_player = Player {
        health: 0,
        mana: None,
        level: 9,
    };
    let revived_player = dead_player
        .revive()
        .expect("reviving a dead player must return Some(player)");
    assert_eq!(revived_player.health, 100);
    assert_eq!(revived_player.mana, None);
    assert_eq!(revived_player.level, dead_player.level);
}

#[test]
fn test_reviving_dead_level10_player() {
    let dead_player = Player {
        health: 0,
        mana: Some(0),
        level: 10,
    };
    let revived_player = dead_player
        .revive()
        .expect("reviving a dead player must return Some(player)");
    assert_eq!(revived_player.health, 100);
    assert_eq!(revived_player.mana, Some(100));
    assert_eq!(revived_player.level, dead_player.level);
}

#[test]
fn test_reviving_alive_player() {
    let alive_player = Player {
        health: 1,
        mana: None,
        level: 8,
    };
    assert!(alive_player.revive().is_none());
}

#[test]
fn test_cast_spell_with_enough_mana() {
    const HEALTH: u32 = 99;
    const MANA: u32 = 100;
    const LEVEL: u32 = 100;
    const MANA_COST: u32 = 3;

    let mut accomplished_wizard = Player {
        health: HEALTH,
        mana: Some(MANA),
        level: LEVEL,
    };

    assert_eq!(accomplished_wizard.cast_spell(MANA_COST), MANA_COST * 2);
    assert_eq!(accomplished_wizard.health, HEALTH);
    assert_eq!(accomplished_wizard.mana, Some(MANA - MANA_COST));
    assert_eq!(accomplished_wizard.level, LEVEL);
}

#[test]
fn test_cast_spell_with_insufficient_mana() {
    let mut no_mana_wizard = Player {
        health: 56,
        mana: Some(2),
        level: 22,
    };

    let clone = Player { ..no_mana_wizard };

    assert_eq!(no_mana_wizard.cast_spell(3), 0);
    assert_eq!(no_mana_wizard.health, clone.health);
    assert_eq!(no_mana_wizard.mana, clone.mana);
    assert_eq!(no_mana_wizard.level, clone.level);
}

#[test]
fn test_cast_spell_with_no_mana_pool() {
    const MANA_COST: u32 = 10;

    let mut underleveled_player = Player {
        health: 87,
        mana: None,
        level: 6,
    };

    let clone = Player {
        ..underleveled_player
    };

    assert_eq!(underleveled_player.cast_spell(MANA_COST), 0);
    assert_eq!(underleveled_player.health, clone.health - MANA_COST);
    assert_eq!(underleveled_player.mana, clone.mana);
    assert_eq!(underleveled_player.level, clone.level);
}

#[test]
fn test_cast_large_spell_with_no_mana_pool() {
    const MANA_COST: u32 = 30;

    let mut underleveled_player = Player {
        health: 20,
        mana: None,
        level: 6,
    };

    assert_eq!(underleveled_player.cast_spell(MANA_COST), 0);
    assert_eq!(underleveled_player.health, 0);
    assert_eq!(underleveled_player.mana, None);
    assert_eq!(underleveled_player.level, 6);
}
