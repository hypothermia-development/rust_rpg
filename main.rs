struct class
{
    pub health: i32,
    ac: i32,
    damage: i32

}

struct quests
{
    pub  diff: i32,
    pub enemy_damage: i32,
    pub enemy_health: i32,
    pub num_of_enemies: i32,
    pub  reward_money: i32,
    pub reward_lvl: i32

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

fn get_full_damage_of_quest(enemy_damage: i32, num_of_enemies: i32) -> i32
{
    let return_value: i32 = enemy_damage * num_of_enemies;

    return return_value
}

fn get_full_health_of_quest(enemy_health: i32, num_of_enemies: i32) -> i32
{
    let return_value: i32 = enemy_health * num_of_enemies;

    return return_value
}


fn choose_quest()
{
    println!("Hello!")
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
        enemy_damage: 5,
        enemy_health: 10,
        num_of_enemies: 3,
        reward_money: 30,
        reward_lvl: 1,

    };

    println!("This is the final release of this, the final function to be done is the quest chooser, alongside rewards, and more quests. All will be added in the next update!")

}
