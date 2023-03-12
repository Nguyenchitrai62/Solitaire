use bevy::math::vec2;
use rand::Rng;
use rusty_engine::prelude::*;

struct GameState {
    bich: i32,
    nhep:i32,
    ro:i32,
    co:i32,
    stt:i32,
}
fn main() 
{
    let mut game = Game::new();
    let game_state= GameState { bich:(1), nhep:(1), ro:(1), co:(1), stt:(0) };
    
    let a = game.add_sprite("4", SpritePreset::One1);
    a.collision=true;
    a.scale=0.2;
    a.layer=23.0;
    a.num=4;
    let a = game.add_sprite("5", SpritePreset::One2);
    a.collision=true;
    a.scale=0.2;
    a.layer=23.0;
    a.num=5;
    let a = game.add_sprite("6", SpritePreset::One3);
    a.collision=true;
    a.scale=0.2;
    a.layer=23.0;
    a.num=6;
    let a = game.add_sprite("7", SpritePreset::One4);
    a.collision=true;
    a.scale=0.2;
    a.layer=23.0;
    a.num=7;
    let a = game.add_sprite("8", SpritePreset::Two1);
    a.collision=true;
    a.scale=0.2;
    a.layer=22.0;
    a.num=8;
    let a = game.add_sprite("9", SpritePreset::Two2);
    a.collision=true;
    a.scale=0.2;
    a.layer=22.0;
    a.num=9;
    let a = game.add_sprite("10", SpritePreset::Two3);
    a.collision=true;
    a.scale=0.2;
    a.layer=22.0;
    a.num=10;
    let a = game.add_sprite("11", SpritePreset::Two4);
    a.collision=true;
    a.scale=0.2;
    a.layer=22.0;
    a.num=11;
    let a = game.add_sprite("12", SpritePreset::Three1);
    a.collision=true;
    a.scale=0.2;
    a.layer=21.0;
    a.num=12;
    let a = game.add_sprite("13", SpritePreset::Three2);
    a.collision=true;
    a.scale=0.2;
    a.layer=21.0;
    a.num=13;
    let a = game.add_sprite("14", SpritePreset::Three3);
    a.collision=true;
    a.scale=0.2;
    a.layer=21.0;
    a.num=14;
    let a = game.add_sprite("15", SpritePreset::Three4);
    a.collision=true;
    a.scale=0.2;
    a.layer=21.0;
    a.num=15;
    let a = game.add_sprite("16", SpritePreset::Four1);
    a.collision=true;
    a.scale=0.2;
    a.layer=20.0;
    a.num=16;
    let a = game.add_sprite("17", SpritePreset::Four2);
    a.collision=true;
    a.scale=0.2;
    a.layer=20.0;
    a.num=17;
    let a = game.add_sprite("18", SpritePreset::Four3);
    a.collision=true;
    a.scale=0.2;
    a.layer=20.0;
    a.num=18;
    let a = game.add_sprite("19", SpritePreset::Four4);
    a.collision=true;
    a.scale=0.2;
    a.layer=20.0;
    a.num=19;
    let a = game.add_sprite("20", SpritePreset::Five1);
    a.collision=true;
    a.scale=0.2;
    a.layer=19.0;
    a.num=20;
    let a = game.add_sprite("21", SpritePreset::Five2);
    a.collision=true;
    a.scale=0.2;
    a.layer=19.0;
    a.num=21;
    let a = game.add_sprite("22", SpritePreset::Five3);
    a.collision=true;
    a.scale=0.2;
    a.layer=19.0;
    a.num=22;
    let a = game.add_sprite("23", SpritePreset::Five4);
    a.collision=true;
    a.scale=0.2;
    a.layer=19.0;
    a.num=23;
    let a = game.add_sprite("24", SpritePreset::Six1);
    a.collision=true;
    a.scale=0.2;
    a.layer=18.0;
    a.num=24;
    let a = game.add_sprite("25", SpritePreset::Six2);
    a.collision=true;
    a.scale=0.2;
    a.layer=18.0;
    a.num=25;
    let a = game.add_sprite("26", SpritePreset::Six3);
    a.collision=true;
    a.scale=0.2;
    a.layer=18.0;
    a.num=26;
    let a = game.add_sprite("27", SpritePreset::Six4);
    a.collision=true;
    a.scale=0.2;
    a.layer=18.0;
    a.num=27;
    let a = game.add_sprite("28", SpritePreset::Seven1);
    a.collision=true;
    a.scale=0.2;
    a.layer=17.0;
    a.num=28;
    let a = game.add_sprite("29", SpritePreset::Seven2);
    a.collision=true;
    a.scale=0.2;
    a.layer=17.0;
    a.num=29;
    let a = game.add_sprite("30", SpritePreset::Seven3);
    a.collision=true;
    a.scale=0.2;
    a.layer=17.0;
    a.num=30;
    let a = game.add_sprite("31", SpritePreset::Seven4);
    a.collision=true;
    a.scale=0.2;
    a.layer=17.0;
    a.num=31;
    let a = game.add_sprite("32", SpritePreset::Eight1);
    a.collision=true;
    a.scale=0.2;
    a.layer=16.0;
    a.num=32;
    let a = game.add_sprite("33", SpritePreset::Eight2);
    a.collision=true;
    a.scale=0.2;
    a.layer=16.0;
    a.num=33;
    let a = game.add_sprite("34", SpritePreset::Eight3);
    a.collision=true;
    a.scale=0.2;
    a.layer=16.0;
    a.num=34;
    let a = game.add_sprite("35", SpritePreset::Eight4);
    a.collision=true;
    a.scale=0.2;
    a.layer=16.0;
    a.num=35;
    let a = game.add_sprite("36", SpritePreset::Nine1);
    a.collision=true;
    a.scale=0.2;
    a.layer=15.0;
    a.num=36;
    let a = game.add_sprite("37", SpritePreset::Nine2);
    a.collision=true;
    a.scale=0.2;
    a.layer=15.0;
    a.num=37;
    let a = game.add_sprite("38", SpritePreset::Nine3);
    a.collision=true;
    a.scale=0.2;
    a.layer=15.0;
    a.num=38;
    let a = game.add_sprite("39", SpritePreset::Nine4);
    a.collision=true;
    a.scale=0.2;
    a.layer=15.0;
    a.num=39;
    let a = game.add_sprite("40", SpritePreset::Ten1);
    a.collision=true;
    a.scale=0.2;
    a.layer=14.0;
    a.num=40;
    let a = game.add_sprite("41", SpritePreset::Ten2);
    a.collision=true;
    a.scale=0.2;
    a.layer=14.0;
    a.num=41;
    let a = game.add_sprite("42", SpritePreset::Ten3);
    a.collision=true;
    a.scale=0.2;
    a.layer=14.0;
    a.num=42;
    let a = game.add_sprite("43", SpritePreset::Ten4);
    a.collision=true;
    a.scale=0.2;
    a.layer=14.0;
    a.num=43;
    let a = game.add_sprite("44", SpritePreset::Jack1);
    a.collision=true;
    a.scale=0.2;
    a.layer=13.0;
    a.num=44;
    let a = game.add_sprite("45", SpritePreset::Jack2);
    a.collision=true;
    a.scale=0.2;
    a.layer=13.0;
    a.num=45;
    let a = game.add_sprite("46", SpritePreset::Jack3);
    a.collision=true;
    a.scale=0.2;
    a.layer=13.0;
    a.num=46;
    let a = game.add_sprite("47", SpritePreset::Jack4);
    a.collision=true;
    a.scale=0.2;
    a.layer=13.0;
    a.num=47;
    let a = game.add_sprite("48", SpritePreset::Queen1);
    a.collision=true;
    a.scale=0.2;
    a.layer=12.0;
    a.num=48;
    let a = game.add_sprite("49", SpritePreset::Queen2);
    a.collision=true;
    a.scale=0.2;
    a.layer=12.0;
    a.num=49;
    let a = game.add_sprite("50", SpritePreset::Queen3);
    a.collision=true;
    a.scale=0.2;
    a.layer=12.0;
    a.num=50;
    let a = game.add_sprite("51", SpritePreset::Queen4);
    a.collision=true;
    a.scale=0.2;
    a.layer=12.0;
    a.num=51;
    let a = game.add_sprite("52", SpritePreset::King1);
    a.collision=true;
    a.scale=0.2;
    a.layer=11.0;
    a.num=52;
    let a = game.add_sprite("53", SpritePreset::King2);
    a.collision=true;
    a.scale=0.2;
    a.layer=11.0;
    a.num=53;
    let a = game.add_sprite("54", SpritePreset::King3);
    a.collision=true;
    a.scale=0.2;
    a.layer=11.0;
    a.num=54;
    let a = game.add_sprite("55", SpritePreset::King4);
    a.collision=true;
    a.scale=0.2;
    a.layer=11.0;
    a.num=55;
    



    let a = game.add_sprite("4.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-4;
    a.layer=0.0;
    let a = game.add_sprite("5.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-5;
    a.layer=0.0;
    let a = game.add_sprite("6.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-6;
    a.layer=0.0;
    let a = game.add_sprite("7.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-7;
    a.layer=0.0;
    let a = game.add_sprite("8.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-8;
    a.layer=0.0;
    let a = game.add_sprite("9.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-9;
    a.layer=0.0;
    let a = game.add_sprite("10.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-10;
    a.layer=0.0;
    let a = game.add_sprite("11.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-11;
    a.layer=0.0;
    let a = game.add_sprite("12.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-12;
    a.layer=0.0;
    let a = game.add_sprite("13.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-13;
    a.layer=0.0;
    let a = game.add_sprite("14.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-14;
    a.layer=0.0;
    let a = game.add_sprite("15.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-15;
    a.layer=0.0;
    let a = game.add_sprite("16.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-16;
    a.layer=0.0;
    let a = game.add_sprite("17.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-17;
    a.layer=0.0;
    let a = game.add_sprite("18.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-18;
    a.layer=0.0;
    let a = game.add_sprite("19.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-19;
    a.layer=0.0;
    let a = game.add_sprite("20.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-20;
    a.layer=0.0;
    let a = game.add_sprite("21.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-21;
    a.layer=0.0;
    let a = game.add_sprite("22.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-22;
    a.layer=0.0;
    let a = game.add_sprite("23.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-23;
    a.layer=0.0;
    let a = game.add_sprite("24.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-24;
    a.layer=0.0;
    let a = game.add_sprite("25.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-25;
    a.layer=0.0;
    let a = game.add_sprite("26.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-26;
    a.layer=0.0;
    let a = game.add_sprite("27.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-27;
    a.layer=0.0;
    let a = game.add_sprite("28.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-28;
    a.layer=0.0;
    let a = game.add_sprite("29.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-29;
    a.layer=0.0;
    let a = game.add_sprite("30.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-30;
    a.layer=0.0;
    let a = game.add_sprite("31.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-31;
    a.layer=0.0;
    let a = game.add_sprite("32.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-32;
    a.layer=0.0;
    let a = game.add_sprite("33.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-33;
    a.layer=0.0;
    let a = game.add_sprite("34.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-34;
    a.layer=0.0;
    let a = game.add_sprite("35.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-35;
    a.layer=0.0;
    let a = game.add_sprite("36.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-36;
    a.layer=0.0;
    let a = game.add_sprite("37.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-37;
    a.layer=0.0;
    let a = game.add_sprite("38.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-38;
    a.layer=0.0;
    let a = game.add_sprite("39.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-39;
    a.layer=0.0;
    let a = game.add_sprite("40.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-40;
    a.layer=0.0;
    let a = game.add_sprite("41.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-41;
    a.layer=0.0;
    let a = game.add_sprite("42.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-42;
    a.layer=0.0;
    let a = game.add_sprite("43.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-43;
    a.layer=0.0;
    let a = game.add_sprite("44.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-44;
    a.layer=0.0;
    let a = game.add_sprite("45.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-45;
    a.layer=0.0;
    let a = game.add_sprite("46.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-46;
    a.layer=0.0;
    let a = game.add_sprite("47.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-47;
    a.layer=0.0;
    let a = game.add_sprite("48.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-48;
    a.layer=0.0;
    let a = game.add_sprite("49.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-49;
    a.layer=0.0;
    let a = game.add_sprite("50.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-50;
    a.layer=0.0;
    let a = game.add_sprite("51.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-51;
    a.layer=0.0;
    let a = game.add_sprite("52.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-52;
    a.layer=0.0;
    let a = game.add_sprite("53.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-53;
    a.layer=0.0;
    let a = game.add_sprite("54.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-54;
    a.layer=0.0;
    let a = game.add_sprite("55.1", SpritePreset::Cardback);
    a.collision=true;
    a.scale=0.2;
    a.num=-55;
    a.layer=0.0;
    
    let a = game.add_sprite("bich", SpritePreset::Bich);
    a.translation=vec2(700.0, 300.0);
    a.scale=0.2;
    let a = game.add_sprite("nhep", SpritePreset::Nhep);
    a.translation=vec2(700.0, 100.0);
    a.scale=0.2;
    let a = game.add_sprite("ro", SpritePreset::Ro);
    a.translation=vec2(700.0, -100.0);
    a.scale=0.2;
    let a = game.add_sprite("co", SpritePreset::Co);
    a.translation=vec2(700.0, -300.0);
    a.scale=0.2;


    let a = game.add_sprite("point", SpritePreset::Point);
    a.collision=true;
    a.layer=1.0;
    let a = game.add_sprite("point1", SpritePreset::Point);
    a.translation=vec2(-700.0, 300.0);
    let a = game.add_sprite("khoangcach", SpritePreset::Point);
    a.translation=vec2(-700.0, 300.0);
    let a = game.add_sprite("che", SpritePreset::Che);
    a.translation=vec2(-700.0, 300.0);
    a.layer=100.0;
    let a = game.add_sprite("cardback1", SpritePreset::Cardback1);
    a.translation=vec2(-700.0, -300.0);
    a.collision=true;
    a.scale=0.2;
    let a = game.add_sprite("line1", SpritePreset::Line);
    a.translation=vec2(-450.0, 199.0);
    a.scale=0.2;
    let a = game.add_sprite("line2", SpritePreset::Line);
    a.translation=vec2(-300.0, 199.0);
    a.scale=0.2;
    let a = game.add_sprite("line3", SpritePreset::Line);
    a.translation=vec2(-150.0, 199.0);
    a.scale=0.2;
    let a = game.add_sprite("line4", SpritePreset::Line);
    a.translation=vec2(0.0, 199.0);
    a.scale=0.2;
    let a = game.add_sprite("line5", SpritePreset::Line);
    a.translation=vec2(150.0, 199.0);
    a.scale=0.2;
    let a = game.add_sprite("line6", SpritePreset::Line);
    a.translation=vec2(300.0, 199.0);
    a.scale=0.2;
    let a = game.add_sprite("line7", SpritePreset::Line);
    a.translation=vec2(450.0, 199.0);
    a.scale=0.2;
    let a = game.add_sprite("newgame", SpritePreset::Newgame);
    a.translation=vec2(460.0, 320.0);
    a.scale=0.2;
    
    let mut text = game.add_text("you win", "");
    text.translation =vec2(-50.0, -50.0);

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) 
{
    // đặt các tấm khung ở dưới lá bài cuối cùng
    if let Some(sprite)= engine.sprites.get_mut("line1")
    {
        sprite.translation=vec2(-450.0, 199.0);
        sprite.layer=0.0;
    }
    if let Some(sprite)= engine.sprites.get_mut("line2")
    {
        sprite.translation=vec2(-300.0, 199.0);
        sprite.layer=0.0;
    }
    if let Some(sprite)= engine.sprites.get_mut("line3")
    {
        sprite.translation=vec2(-150.0, 199.0);
        sprite.layer=0.0;
    }
    if let Some(sprite)= engine.sprites.get_mut("line4")
    {
        sprite.translation=vec2(-0.0, 199.0);
        sprite.layer=0.0;
    }
    if let Some(sprite)= engine.sprites.get_mut("line5")
    {
        sprite.translation=vec2(150.0, 199.0);
        sprite.layer=0.0;
    }
    if let Some(sprite)= engine.sprites.get_mut("line6")
    {
        sprite.translation=vec2(300.0, 199.0);
        sprite.layer=0.0;
    }
    if let Some(sprite)= engine.sprites.get_mut("line7")
    {
        sprite.translation=vec2(450.0, 199.0);
        sprite.layer=0.0;
    }


    let text = engine.texts.get_mut("you win").unwrap();
    //tạo màn chơi ngẫu nhiên (chạy 1 lần)
    let mut kiemtra=1.0;
    if let Some(sprite) = engine.sprites.get_mut("point")
    {
        kiemtra=sprite.layer;
    }
    
    if kiemtra==1.0
    {
        game_state.bich=1;
        game_state.nhep=1;
        game_state.ro=1;
        game_state.co=1;
        game_state.stt=0;
        text.value="".into();
        for sprite in engine.sprites.values_mut()
        {
            if sprite.num>3
            {
                let k=sprite.num/4;
                let l=k as f32;
                sprite.layer=24.0-l;
            }
            sprite.stt=-1;
        }
        if let Some(sprite)=engine.sprites.get_mut("cardback1")
        {
            sprite.translation=vec2(-700.0, -300.0);
        }
        let mut arr:[i32;52]=   [4,5,6,7,8,9,10,
                        11,12,13,14,15,16,17,18,19,20,
                        21,22,23,24,25,26,27,28,29,30,
                        31,32,33,34,35,36,37,38,39,40,
                        41,42,43,44,45,46,47,48,49,50,
                        51,52,53,54,55];
        //đảo vị trí mảng ngãu nhiên
        for i in 0..52
        {
            let j= rand::thread_rng().gen_range(0..52);
            let temp=arr[i];
            arr[i]=arr[j];
            arr[j]=temp;
        }
        let arr1=&arr[0..1];
        let arr2=&arr[1..3];
        let arr3=&arr[3..6];
        let arr4=&arr[6..10];
        let arr5=&arr[10..15];
        let arr6=&arr[15..21];
        let arr7=&arr[21..28];
        let arr8=&arr[28..52];
        //hiện thị
        for i in 0..arr.len()
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==arr[i]
                {
                    sprite.translation=vec2(-700.0, 300.0);
                }
            }
        }
  
        for i in 0..arr1.len()
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-arr1[i]
                {
                    let j=i as f32;
                    sprite.translation=vec2(-450.0, 200.0-j*20.0);
                    sprite.layer=j+1.0;
                }
            }
        }
        for i in 0..arr2.len()
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-arr2[i]
                {
                    let j=i as f32;
                    sprite.translation=vec2(-300.0, 200.0-j*20.0);
                    sprite.layer=j+1.0;
                }
            }
        }
        for i in 0..arr3.len()
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-arr3[i]
                {
                    let j=i as f32;
                    sprite.translation=vec2(-150.0, 200.0-j*20.0);
                    sprite.layer=j+1.0;
                }
            }
        }
        for i in 0..arr4.len()
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-arr4[i]
                {
                    let j=i as f32;
                    sprite.translation=vec2(0.0, 200.0-j*20.0);
                    sprite.layer=j+1.0;
                }
            }
        }
        for i in 0..arr5.len()
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-arr5[i]
                {
                    let j=i as f32;
                    sprite.translation=vec2(150.0, 200.0-j*20.0);
                    sprite.layer=j+1.0;
                }
            }
        }
        for i in 0..arr6.len()
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-arr6[i]
                {
                    let j=i as f32;
                    sprite.translation=vec2(300.0, 200.0-j*20.0);
                    sprite.layer=j+1.0;
                }
            }
        }
          for i in 0..arr7.len()
          {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-arr7[i]
                {
                    let j=i as f32;
                    sprite.translation=vec2(450.0, 200.0-j*20.0);
                    sprite.layer=j+1.0;
                }
            }
        }
        for i in 0..arr8.len()
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==arr8[i]
                {
                    let j=i as i32;
                    sprite.stt=j;
                }
                if sprite.num==-arr8[i]
                {
                    sprite.translation=vec2(-700.0, 300.0);
                }
            }
        }
  
        if let Some(sprite) = engine.sprites.get_mut("point")
        {
            sprite.layer=0.0;
        }
    }


    for sprite in engine.sprites.values_mut()
    {
        if sprite.translation!=vec2(-700.0, -150.0)
        {
            sprite.check=false;
        }
    }

    //check lá bài cột 1
    let mut j=0.0;
    while j<=20.0
    {
        let mut k=0.0;
        let mut number=0;
        for sprite in engine.sprites.values_mut()
        {
            if sprite.translation==vec2(-450.0, -200.0+j*20.0) 
            {
                k=j;
                j=30.0;
                sprite.check=true;
                if sprite.num<0
                {
                    number=sprite.num;
                    sprite.translation=vec2(-700.0, 300.0);
                }
            }
        }
        if number!=0
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-number
                {
                    sprite.translation=vec2(-450.0, -200.0+k*20.0);
                    sprite.collision=true;
                }  
            }
        }
        j+=1.0;
    }
    //check lá bài cột 2
    let mut j=0.0;
    while j<=20.0
    {
        let mut k=0.0;
        let mut number=0;
        for sprite in engine.sprites.values_mut()
        {
            if sprite.translation==vec2(-300.0, -200.0+j*20.0) 
            {
                k=j;
                j=30.0;
                sprite.check=true;
                if sprite.num<0
                {
                    number=sprite.num;
                    sprite.translation=vec2(-700.0, 300.0);
                }
            }
        }
        if number!=0
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-number
                {
                    sprite.translation=vec2(-300.0, -200.0+k*20.0);
                    sprite.collision=true;
                }  
            }
        }
        j+=1.0;
    }
    //check lá bài cột 3
    let mut j=0.0;
    while j<=20.0
    {
        let mut k=0.0;
        let mut number=0;
        for sprite in engine.sprites.values_mut()
        {
            if sprite.translation==vec2(-150.0, -200.0+j*20.0) 
            {
                k=j;
                j=30.0;
                sprite.check=true;
                if sprite.num<0
                {
                    number=sprite.num;
                    sprite.translation=vec2(-700.0, 300.0);
                }
            }
        }
        if number!=0
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-number
                {
                    sprite.translation=vec2(-150.0, -200.0+k*20.0);
                    sprite.collision=true;
                }  
            }
        }
        j+=1.0;
    }
    //check lá bài cột 4
    let mut j=0.0;
    while j<=20.0
    {
        let mut k=0.0;
        let mut number=0;
        for sprite in engine.sprites.values_mut()
        {
            if sprite.translation==vec2(0.0, -200.0+j*20.0) 
            {
                k=j;
                j=30.0;
                sprite.check=true;
                if sprite.num<0
                {
                    number=sprite.num;
                    sprite.translation=vec2(-700.0, 300.0);
                }
            }
        }
        if number!=0
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-number
                {
                    sprite.translation=vec2(0.0, -200.0+k*20.0);
                    sprite.collision=true;
                }  
            }
        }
        j+=1.0;
    }
    //check lá bài cột 5
    let mut j=0.0;
    while j<=20.0
    {
        let mut k=0.0;
        let mut number=0;
        for sprite in engine.sprites.values_mut()
        {
            if sprite.translation==vec2(150.0, -200.0+j*20.0) 
            {
                k=j;
                j=30.0;
                sprite.check=true;
                if sprite.num<0
                {
                    number=sprite.num;
                    sprite.translation=vec2(-700.0, 300.0);
                }
            }
        }
        if number!=0
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-number
                {
                    sprite.translation=vec2(150.0, -200.0+k*20.0);
                    sprite.collision=true;
                }  
            }
        }
        j+=1.0;
    }
    //check lá bài cột 6
    let mut j=0.0;
    while j<=20.0
    {
        let mut k=0.0;
        let mut number=0;
        for sprite in engine.sprites.values_mut()
        {
            if sprite.translation==vec2(300.0, -200.0+j*20.0) 
            {
                k=j;
                j=30.0;
                sprite.check=true;
                if sprite.num<0
                {
                    number=sprite.num;
                    sprite.translation=vec2(-700.0, 300.0);
                }
            }
        }
        if number!=0
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-number
                {
                    sprite.translation=vec2(300.0, -200.0+k*20.0);
                    sprite.collision=true;
                }  
            }
        }
        j+=1.0;
    }
    //check lá bài cột 7
    let mut j=0.0;
    while j<=20.0
    {
        let mut k=0.0;
        let mut number=0;
        for sprite in engine.sprites.values_mut()
        {
            if sprite.translation==vec2(450.0, -200.0+j*20.0) 
            {
                k=j;
                j=30.0;
                sprite.check=true;
                if sprite.num<0
                {
                    number=sprite.num;
                    sprite.translation=vec2(-700.0, 300.0);
                }
            }
        }
        if number!=0
        {
            for sprite in engine.sprites.values_mut()
            {
                if sprite.num==-number
                {
                    sprite.translation=vec2(450.0, -200.0+k*20.0);
                    sprite.collision=true;
                }  
            }
        }
        j+=1.0;
    }

  

    //nếu ko còn lá bài nào có thể rút thì xóa bỏ hình lá bài ở góc dưới bên trái
    let mut check=false;
    for sprite in engine.sprites.values_mut()
    {
        for i in 0..24
        {
            if sprite.stt==i && sprite.translation==vec2(-700.0, 300.0)
            {
                check=true;
            }
        }
    }
    if check==false
    {
        if let Some(sprite) = engine.sprites.get_mut("cardback1")
        {
            sprite.translation=vec2(-700.0, 300.0)
        }
    }
    
    //tìm lá bài khả dụng để xuất hiện khi bấm
    if check==true
    {
        let mut check1=false;
        for sprite in engine.sprites.values_mut()
        {
            if sprite.stt==game_state.stt && sprite.translation==vec2(-700.0, 300.0)
            {
                check1=true;
            }
        }
        if check1==false
        {
            game_state.stt+=1;
            if game_state.stt>23
            {
                game_state.stt=0;
            }
        }
    }
    


    //gán 1 sprite vào con trỏ
    if let Some(sprite) = engine.sprites.get_mut("point")
    {
        if let Some(location)=engine.mouse_state.location()
        {
            sprite.translation=location;
        }
    }

    //kiểm tra va chạm
    for event in engine.collision_events.drain(..) 
    {
        //nếu va chạm không có "point" thì bỏ qua
        if event.pair.0=="point" || event.pair.1=="point"
        {
        
        match event.state 
        {
            CollisionState::Begin => 
            {
                //đánh dấu những sprite đang va chạm
                if let Some(sprite) = engine.sprites.get_mut(&event.pair.0)
                {
                    sprite.mark=true;
                }
                if let Some(sprite) = engine.sprites.get_mut(&event.pair.1)
                {
                    sprite.mark=true;
                }
            }
            CollisionState::End => 
            {
                if let Some(sprite) = engine.sprites.get_mut(&event.pair.0)
                {
                    sprite.mark=false;
                }
                if let Some(sprite) = engine.sprites.get_mut(&event.pair.1)
                {
                    sprite.mark=false;
                }
            }
        }
        }
    }
    
    //tìm sprite có layer cao nhất
    let mut khoangcach=vec2(0.0, 0.0);
    let mut temp=0.0;
    for sprite in engine.sprites.values_mut()
    {
        if sprite.mark==true && sprite.label!= "point" && sprite.layer>temp
        {
            temp=sprite.layer;
            khoangcach=sprite.translation;
        }
    }
   
    if engine.mouse_state.just_pressed(MouseButton::Left)
    {
        //tính khoảng cách con trỏ với sprite
        if let Some(sprite)=engine.sprites.get_mut("point1")
        {
            sprite.translation=khoangcach;
        }
        if let Some(location)=engine.mouse_state.location()
        {
            khoangcach-=location;
        }
        if let Some(sprite)=engine.sprites.get_mut("khoangcach")
        {
            sprite.translation=khoangcach;
        }
        
        //nếu click vào lá bài ở bên dưới góc trái xuất hiện lá bài
        if let Some(sprite)= engine.sprites.get_mut("cardback1")
        {
            if sprite.mark==true
            {
                for sprite in engine.sprites.values_mut()
                {
                    if sprite.stt==game_state.stt && sprite.translation==vec2(-700.0, 300.0)
                    {
                        sprite.translation=vec2(-700.0, -150.0);
                        sprite.check=true;
                    }
                    if sprite.stt!=game_state.stt && sprite.translation==vec2(-700.0, -150.0)
                    {
                        sprite.translation=vec2(-700.0, 300.0);
                    }
                }
            }
        }
        
        //nếu click vào new game 
        let mut kiemtra=0.0;
        if let Some(sprite)=engine.sprites.get_mut("newgame")
        {
            if sprite.mark==true
            {
                kiemtra=1.0;
            } 
        }
        if let Some(sprite)=engine.sprites.get_mut("point")
        {
            sprite.layer=kiemtra;
        }
        
        //đặt tất cả các sprite không liên quan thì không có phản ứng
        for sprite in engine.sprites.values_mut()
        {
            if sprite.layer!=temp && sprite.label!="point" 
            {
                sprite.collision=false;
            }
            if sprite.mark==false && sprite.layer==temp
            {
                sprite.collision=false;
            }
        }
    }

    if engine.mouse_state.pressed(MouseButton::Left)
    {
        if let Some(sprite)=engine.sprites.get_mut("khoangcach")
        {
            khoangcach=sprite.translation;
        }
        
        let mut bandau=vec2(1.0, 1.0);
        for sprite in engine.sprites.values_mut()
        {
            //dịch chuyển sprite đến vị trí mới
            if sprite.layer==temp && sprite.label!="point" && sprite.collision==true && sprite.label!="cardback1" && sprite.label!="newgame"
            {
                bandau=sprite.translation;
                // Kiểm tra xem lá bài có thỏa mãn để xếp ra ngoài hay không
                if sprite.num%4==0 && sprite.check==true
                {
                    let k=sprite.num/4;
                    if k==game_state.bich
                    {
                        sprite.translation=vec2(700.0, 300.0);
                        let l=k as f32;
                        sprite.layer=l;
                        game_state.bich+=1;
                    }
                }
                if sprite.num%4==1 && sprite.check==true
                {
                    let k=sprite.num/4;
                    if k==game_state.nhep
                    {
                        sprite.translation=vec2(700.0, 100.0);
                        let l=k as f32;
                        sprite.layer=l;
                        game_state.nhep+=1;
                    }
                }
                if sprite.num%4==2 && sprite.check==true
                {
                    let k=sprite.num/4;
                    if k==game_state.ro
                    {
                        sprite.translation=vec2(700.0, -100.0);
                        let l=k as f32;
                        sprite.layer=l;
                        game_state.ro+=1;
                    }
                }
                if sprite.num%4==3 && sprite.check==true
                {
                    let k=sprite.num/4;
                    if k==game_state.co
                    {
                        sprite.translation=vec2(700.0, -300.0);
                        let l=k as f32;
                        sprite.layer=l;
                        game_state.co+=1;
                    }
                }
            }
        }

        //tăng layer của lá bài khi dc bấm
        if bandau!=vec2(1.0, 1.0)
        {
            for sprite in engine.sprites.values_mut()
            {
                for i in 0..13
                {
                    let j=i as f32;
                    if sprite.translation==bandau-vec2(0.0, 20.0*j) && sprite.label!="point1"
                    {
                        if let Some(location)=engine.mouse_state.location()
                        {
                            if sprite.layer<30.0
                            {
                                sprite.layer+=100.0;
                            }
                            if temp<30.0
                            {
                                temp+=100.0;
                            }
                            sprite.translation=location+khoangcach-vec2(0.0, 20.0*j);
                        }
                    }
                }
            }
        }
    }

    if engine.mouse_state.just_released(MouseButton::Left)
    {
        //kiểm tra vị trí mới
        let mut bandau=vec2(0.0, 0.0);
        let mut spritebandau=vec2(1.0, 0.0);
        let mut number=0;
        let mut a=-1;
        let mut b=-2;
        let mut check=false;
        for sprite in engine.sprites.values_mut()
        {
            if sprite.layer==temp && sprite.label!="point" && sprite.collision==true && sprite.label!="cardback1" && sprite.label!="newgame"
            {
                spritebandau=sprite.translation;
                number=sprite.num;
                
                if sprite.num%4==0 || sprite.num%4==1
                {
                    a=0;
                }
                if sprite.num%4==2 || sprite.num%4==3
                {
                    a=1;
                }

                if sprite.num/4==13
                {
                    if  bandau==vec2(-450.0, 200.0) ||
                        bandau==vec2(-300.0, 200.0) ||
                        bandau==vec2(-150.0, 200.0) ||
                        bandau==vec2(0.0, 200.0) ||
                        bandau==vec2(150.0, 200.0) ||
                        bandau==vec2(300.0, 200.0) ||
                        bandau==vec2(450.0, 200.0) 
                    {
                        check=true;
                    }
                }
            }
        }

        //check màu 2 lá bài là số 2 lá bài
        for sprite in engine.sprites.values_mut()
        {
            if sprite.check ==true && bandau==vec2(0.0, 0.0)
            {
                if sprite.num%4==0 || sprite.num%4==1
                {
                    b=0;
                }
                if sprite.num%4==2 || sprite.num%4==3
                {   
                    b=1;
                }
                if sprite.num/4-number/4==1 && a!=b && a!=-1 && b!=-2
                {
                    bandau=sprite.translation-vec2(0.0, 20.0)-spritebandau;
                    if bandau.x*bandau.x+bandau.y*bandau.y<6400.0
                    {
                        check=true;
                        bandau=sprite.translation-vec2(0.0, 20.0);
                    }else 
                    {
                        bandau=vec2(0.0, 0.0);
                    }
                }
            }   
        }

        //nếu là lá K thì kiểm tra
        if number/4==13 && bandau==vec2(0.0, 0.0)
        {
            bandau=vec2(-450.0, 200.0)-spritebandau;
            if bandau.x*bandau.x+bandau.y*bandau.y<6400.0
            {
                check=true;
                bandau=vec2(-450.0, 200.0);
            }else 
            {
                bandau=vec2(0.0, 0.0);
            }
        }
        if number/4==13 && bandau==vec2(0.0, 0.0)
        {
            bandau=vec2(-300.0, 200.0)-spritebandau;
            if bandau.x*bandau.x+bandau.y*bandau.y<6400.0
            {
                check=true;
                bandau=vec2(-300.0, 200.0);
            }else 
            {
                bandau=vec2(0.0, 0.0);
            }
        }
        if number/4==13 && bandau==vec2(0.0, 0.0)
        {
            bandau=vec2(-150.0, 200.0)-spritebandau;
            if bandau.x*bandau.x+bandau.y*bandau.y<6400.0
            {
                check=true;
                bandau=vec2(-150.0, 200.0);
            }else 
            {
                bandau=vec2(0.0, 0.0);
            }
        }
        if number/4==13 && bandau==vec2(0.0, 0.0)
        {
            bandau=vec2(0.0, 200.0)-spritebandau;
            if bandau.x*bandau.x+bandau.y*bandau.y<6400.0
            {
                check=true;
                bandau=vec2(0.0, 200.0);
            }else 
            {
                bandau=vec2(0.0, 0.0);
            }
        }
        if number/4==13 && bandau==vec2(0.0, 0.0)
        {
            bandau=vec2(150.0, 200.0)-spritebandau;
            if bandau.x*bandau.x+bandau.y*bandau.y<6400.0
            {
                check=true;
                bandau=vec2(150.0, 200.0);
            }else 
            {
                bandau=vec2(0.0, 0.0);
            }
        }
        if number/4==13 && bandau==vec2(0.0, 0.0)
        {
            bandau=vec2(300.0, 200.0)-spritebandau;
            if bandau.x*bandau.x+bandau.y*bandau.y<6400.0
            {
                check=true;
                bandau=vec2(300.0, 200.0);
            }else 
            {
                bandau=vec2(0.0, 0.0);
            }
        }
        if number/4==13 && bandau==vec2(0.0, 0.0)
        {
            bandau=vec2(450.0, 200.0)-spritebandau;
            if bandau.x*bandau.x+bandau.y*bandau.y<6400.0
            {
                check=true;
                bandau=vec2(450.0, 200.0);
            }else 
            {
                bandau=vec2(0.0, 0.0);
            }
        }
        for sprite in engine.sprites.values_mut()
        {
            if sprite.translation==bandau
            {
                check=false;
            }
        }

        //thực hiện di chuyển sprite
        if check==true
        {
            for sprite in engine.sprites.values_mut()
            {
                for i in 0..13
                {
                    let j=i as f32;
                    if sprite.translation==spritebandau-vec2(0.0, 20.0*j) && sprite.label!="point1"
                    {
                        sprite.translation=bandau-vec2(0.0, 20.0*j);
                        sprite.layer-=100.0;
                    }
                }
            }
        }
        
        //nếu vị trí mới ko thỏa mãn thì đưa sprite về vị trí ban đầu
        if check==false
        {
            let mut bandau=vec2(0.0,0.0);
            if let Some(sprite)=engine.sprites.get_mut("point1")
            {
                bandau=sprite.translation;
            }
            for sprite in engine.sprites.values_mut()
            {
                for i in 0..13
                {
                    let j=i as f32;
                    if sprite.translation==spritebandau-vec2(0.0, 20.0*j) && sprite.label!="point1" && sprite.layer>=temp
                    {
                        sprite.translation=bandau-vec2(0.0, 20.0*j);
                        sprite.layer-=100.0;
                    }
                }
            }
        }

        if let Some(sprite)=engine.sprites.get_mut("point1")
        {
            sprite.translation=vec2(-700.0, 300.0);
        }
        for sprite in engine.sprites.values_mut()
        {
            if  sprite.translation!=vec2(700.0, 300.0) &&
                sprite.translation!=vec2(700.0, 100.0) &&
                sprite.translation!=vec2(700.0, -100.0) &&
                sprite.translation!=vec2(700.0, -300.0) &&
                sprite.translation!=vec2(-700.0,300.0)
            {
                sprite.collision=true;
            }
        }
    }
   
    //kiểm tra chiến thắng
    if game_state.bich==14 && game_state.nhep==14 && game_state.ro==14 && game_state.co==14   
    {
        text.value="You Win!".into();
        text.font_size = 120.0;
    }
  
}
