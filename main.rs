struct class
{
    pub health: i32,
    ac: i32,
    damage: i32

}

struct quests
{
    pub  diff: i32,
    pub  reward_money: i32,
    pub  reward_xp: i32,

}


fn attack(opponent_health: i32, player_damage: i32)  -> i32
{
    let mut n: i32 = 0;

    n = opponent_health - player_damage;
    
    return n
}


fn on_lvl_up_hp(starting_hp: i32) -> i32
{   
    let return_value: i32 = starting_hp + 15;

    return return_value
}
fn on_lvl_up_dmg(starting_dmg: i32) -> i32
{
    let return_value: i32 = starting_dmg + 15;

    return return_value
}
fn main() {

    // class init
    let fighter = class 
    {
        health: 30,
        damage: 15,
        ac: 0
    };
    let wizard = class
    {
        health:15,
        damage: 30,
        ac: 0
    };

    let goblin = quests
    {
        diff:1,
        reward_money: 30,
        reward_xp: 30,

    };

}
