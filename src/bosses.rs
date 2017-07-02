use self::Language::*;

macro_rules! boss {
    ($name:expr, $level:expr, $language:ident, $url:expr) => {
        Boss {
            name: $name,
            level: $level,
            language: $language,
            url: $url
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Language {
    En,
    Ja,
}

#[derive(Debug, PartialEq)]
pub struct Boss {
    pub name: &'static str,
    pub level: u8,
    pub language: Language,
    pub url: &'static str,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const BOSSES: &[Boss] = &[
    //
    // Japanese bosses
    //

    // Guild Wars
    boss!("Lv30 アーフラー", 30, Ja, "https://pbs.twimg.com/media/CeSO4quUYAAy8U1.jpg"),
    boss!("Lv40 アーフラー", 40, Ja, "https://pbs.twimg.com/media/CeSO6aAWAAAAOGe.jpg"),
    boss!("Lv40 ゲイザー", 40, Ja, "https://pbs.twimg.com/media/Cn7opV5VUAAfgDz.jpg"),
    boss!("Lv40 ヨグ＝ソトース", 40, Ja, "https://pbs.twimg.com/media/Cqlrx16VUAAU1-Z.jpg"),
    boss!("Lv50 ベオウルフ", 50, Ja, "https://pbs.twimg.com/media/CeSO8gHWAAEnMGl.jpg"),
    boss!("Lv60 グガランナ", 60, Ja, "https://pbs.twimg.com/media/Cn7o2sYVYAEP45o.jpg"),
    boss!("Lv60 ベオウルフ", 60, Ja, "https://pbs.twimg.com/media/CeSO-QrWwAECCG1.jpg"),
    boss!("Lv60 マヒシャ", 60, Ja, "https://pbs.twimg.com/media/Cqlr4E5VMAAOGqb.jpg"),
    boss!("Lv75 エメラルドホーン", 75, Ja, "https://pbs.twimg.com/media/Cn7o767UkAEX8-H.jpg"),
    boss!("Lv75 スーペルヒガンテ", 75, Ja, "https://pbs.twimg.com/media/Cqlr_RRVYAEabiO.jpg"),

    // Standard
    boss!("Lv30 クレイゴーレム", 30, Ja, "https://pbs.twimg.com/media/Crtprh7UIAARax2.jpg"),
    boss!("Lv30 コロッサス", 30, Ja, "https://pbs.twimg.com/media/CT6cUf8VEAEBaEb.jpg"),
    boss!("Lv30 セレスト", 30, Ja, "https://pbs.twimg.com/media/CT6cmzjUcAIvSo_.jpg"),
    boss!("Lv30 ティアマト", 30, Ja, "https://pbs.twimg.com/media/CT6cLx4VAAAzePV.jpg"),
    boss!("Lv50 アドウェルサ", 50, Ja, "https://pbs.twimg.com/media/CT6cgc5UYAIhJoB.jpg"),
    boss!("Lv50 コロッサス", 50, Ja, "https://pbs.twimg.com/media/CT6cUf8VEAEBaEb.jpg"),
    boss!("Lv50 セレスト", 50, Ja, "https://pbs.twimg.com/media/CT6cmzjUcAIvSo_.jpg"),
    boss!("Lv50 ティアマト", 50, Ja, "https://pbs.twimg.com/media/CT6cLx4VAAAzePV.jpg"),
    boss!("Lv50 ティアマト・マグナ", 50, Ja, "https://pbs.twimg.com/media/CT6buTPUwAIM9VJ.jpg"),
    boss!("Lv50 ユグドラシル", 50, Ja, "https://pbs.twimg.com/media/CT6cbScU8AAgXRw.jpg"),
    boss!("Lv50 リヴァイアサン", 50, Ja, "https://pbs.twimg.com/media/CT6cXcgUEAEc0Zl.jpg"),
    boss!("Lv50 ヴェセラゴ", 50, Ja, "https://pbs.twimg.com/media/Crtpt5RUAAAV6OG.jpg"),
    boss!("Lv60 ユグドラシル・マグナ", 60, Ja, "https://pbs.twimg.com/media/CT6cDD3UkAEnP8Y.jpg"),
    boss!("Lv60 リヴァイアサン・マグナ", 60, Ja, "https://pbs.twimg.com/media/CT6cBK3U8AA4xdW.jpg"),
    boss!("Lv70 コロッサス・マグナ", 70, Ja, "https://pbs.twimg.com/media/CT6bwJTUYAA6mcV.jpg"),
    boss!("Lv75 シュヴァリエ・マグナ", 75, Ja, "https://pbs.twimg.com/media/CT6cEwEUcAAlwFM.jpg"),
    boss!("Lv75 セレスト・マグナ", 75, Ja, "https://pbs.twimg.com/media/CT6cGF4UYAAHBg5.jpg"),
    boss!("Lv100 Dエンジェル・オリヴィエ", 100, Ja, "https://pbs.twimg.com/media/CT6csqNVAAA_GFU.jpg"),
    boss!("Lv100 アテナ", 100, Ja, "https://pbs.twimg.com/media/Cg7oAJsUkAApRif.jpg"),
    boss!("Lv100 アポロン", 100, Ja, "https://pbs.twimg.com/media/CT6chwtUsAA0WFw.jpg"),
    boss!("Lv100 オーディン", 100, Ja, "https://pbs.twimg.com/media/CqwDU_jUkAQjgKq.jpg"),
    boss!("Lv100 ガルーダ", 100, Ja, "https://pbs.twimg.com/media/CkVbhuqVEAA7e6K.jpg"),
    boss!("Lv100 グラニ", 100, Ja, "https://pbs.twimg.com/media/CqwDXDkUMAAjCE5.jpg"),
    boss!("Lv100 コロッサス・マグナ", 100, Ja, "https://pbs.twimg.com/media/CVL2CmeUwAAElDW.jpg"),
    boss!("Lv100 シュヴァリエ・マグナ", 100, Ja, "https://pbs.twimg.com/media/CbTeQ1fVIAAEoqi.jpg"),
    boss!("Lv100 ジ・オーダー・グランデ", 100, Ja, "https://pbs.twimg.com/media/CdL4YeiUEAI0JKW.jpg"),
    boss!("Lv100 セレスト・マグナ", 100, Ja, "https://pbs.twimg.com/media/CbTeWuMUUAIzFZl.jpg"),
    boss!("Lv100 ティアマト・マグナ＝エア", 100, Ja, "https://pbs.twimg.com/media/CT6cNUBUAAETdz6.jpg"),
    boss!("Lv100 ナタク", 100, Ja, "https://pbs.twimg.com/media/CT6cOzzUwAESsq_.jpg"),
    boss!("Lv100 バアル", 100, Ja, "https://pbs.twimg.com/media/CjLxgrbUgAAFbwi.jpg"),
    boss!("Lv100 フラム＝グラス", 100, Ja, "https://pbs.twimg.com/media/CT_qpfCUsAA9vfF.jpg"),
    boss!("Lv100 プロトバハムート", 100, Ja, "https://pbs.twimg.com/media/CT6cIKmUYAAPVmD.jpg"),
    boss!("Lv100 マキュラ・マリウス", 100, Ja, "https://pbs.twimg.com/media/CT6cYp-UsAAy_f0.jpg"),
    boss!("Lv100 メドゥーサ", 100, Ja, "https://pbs.twimg.com/media/CT6ccesVEAAy_Kx.jpg"),
    boss!("Lv100 ユグドラシル・マグナ", 100, Ja, "https://pbs.twimg.com/media/CYBkhTmUsAAPjgu.jpg"),
    boss!("Lv100 リッチ", 100, Ja, "https://pbs.twimg.com/media/CqwDZKwVMAAxt0Y.jpg"),
    boss!("Lv100 リヴァイアサン・マグナ", 100, Ja, "https://pbs.twimg.com/media/CYBkbZSUAAA2BW-.jpg"),
    boss!("Lv110 ローズクイーン", 110, Ja, "https://pbs.twimg.com/media/CUnrstgUYAA4hOz.jpg"),
    boss!("Lv120 Dエンジェル・オリヴィエ", 120, Ja, "https://pbs.twimg.com/media/CbTeSqbUcAARoNV.jpg"),
    boss!("Lv120 アポロン", 120, Ja, "https://pbs.twimg.com/media/CbTeO4fUkAEmmIN.jpg"),
    boss!("Lv120 ギルガメッシュ", 120, Ja, "https://pbs.twimg.com/media/CqG0X1tUkAA5B8_.jpg"),
    boss!("Lv120 ナタク", 120, Ja, "https://pbs.twimg.com/media/CT6cQD-UcAE3nt2.jpg"),
    boss!("Lv120 フラム＝グラス", 120, Ja, "https://pbs.twimg.com/media/CVL2EBHUwAA8nUj.jpg"),
    boss!("Lv120 マキュラ・マリウス", 120, Ja, "https://pbs.twimg.com/media/CYBkd_1UEAATH9J.jpg"),
    boss!("Lv120 メドゥーサ", 120, Ja, "https://pbs.twimg.com/media/CYBki-CUkAQVWW_.jpg"),
    boss!("Lv150 プロトバハムート", 150, Ja, "https://pbs.twimg.com/media/CdL4WyxUYAIXPb8.jpg"),

    //
    // English bosses
    //

    // Guild Wars
    boss!("Lvl 30 Ahura", 30, En, "https://pbs.twimg.com/media/Cs1w7-QUIAApnHh.jpg"),
    boss!("Lvl 40 Ahura", 40, En, "https://pbs.twimg.com/media/Cs1w9nhVIAABgqg.jpg"),
    boss!("Lvl 40 Ogler", 40, En, "https://pbs.twimg.com/media/Cn7oqbPUkAEimW2.jpg"),
    boss!("Lvl 40 Yog-Sothoth", 40, En, "https://pbs.twimg.com/media/Cqlr0pRVUAQxZ5g.jpg"),
    boss!("Lvl 50 Grendel", 50, En, "https://pbs.twimg.com/media/Cs1w-68UEAEjhEV.jpg"),
    boss!("Lvl 60 Grendel", 60, En, "https://pbs.twimg.com/media/Cs1xAKcVIAEXC9a.jpg"),
    boss!("Lvl 60 Gugalanna", 60, En, "https://pbs.twimg.com/media/Cn7o6ppVUAArahp.jpg"),
    boss!("Lvl 60 Mahisha", 60, En, "https://pbs.twimg.com/media/Cqlr51CUAAA9lp3.jpg"),
    boss!("Lvl 75 Supergigante", 75, En, "https://pbs.twimg.com/media/CqlsAzGUkAAAcRT.jpg"),
    boss!("Lvl 75 Viridian Horn", 75, En, "https://pbs.twimg.com/media/Cn7o85cUEAAnlh0.jpg"),

    // Standard
    boss!("Lvl 50 Celeste", 50, En, "https://pbs.twimg.com/media/CfqXLhHUEAAF92L.jpg"),
    boss!("Lvl 50 Leviathan", 50, En, "https://pbs.twimg.com/media/CfqW-MVUIAEJrDP.jpg"),
    boss!("Lvl 50 Tiamat Omega", 50, En, "https://pbs.twimg.com/media/CfqXQA3UMAEMV7O.jpg"),
    boss!("Lvl 50 Tiamat", 50, En, "https://pbs.twimg.com/media/CfqWy2tUUAAr9yu.jpg"),
    boss!("Lvl 50 Veselago", 50, En, "https://pbs.twimg.com/media/Crtpu8RVMAALBKk.jpg"),
    boss!("Lvl 50 Yggdrasil", 50, En, "https://pbs.twimg.com/media/CfqXDAQVIAAixiA.jpg"),
    boss!("Lvl 60 Leviathan Omega", 60, En, "https://pbs.twimg.com/media/CfqXTAQUAAAu3ox.jpg"),
    boss!("Lvl 60 Yggdrasil Omega", 60, En, "https://pbs.twimg.com/media/CfuZgxLUkAArdGe.jpg"),
    boss!("Lvl 70 Colossus Omega", 70, En, "https://pbs.twimg.com/media/CfqXRjsUAAAwXTP.jpg"),
    boss!("Lvl 75 Celeste Omega", 75, En, "https://pbs.twimg.com/media/CfqXXVDUUAA0hAS.jpg"),
    boss!("Lvl 75 Luminiera Omega", 75, En, "https://pbs.twimg.com/media/CfqXWAhUsAAprzd.jpg"),
    boss!("Lvl 100 Apollo", 100, En, "https://pbs.twimg.com/media/CfqXI0lVAAAQgcj.jpg"),
    boss!("Lvl 100 Athena", 100, En, "https://pbs.twimg.com/media/Cg7oBQ_UYAEAIK7.jpg"),
    boss!("Lvl 100 Baal", 100, En, "https://pbs.twimg.com/media/CjLxhwmUoAEglVC.jpg"),
    boss!("Lvl 100 Celeste Omega", 100, En, "https://pbs.twimg.com/media/CfqZzDsUUAA5DEX.jpg"),
    boss!("Lvl 100 Colossus Omega", 100, En, "https://pbs.twimg.com/media/CfqZOt6VIAAniVV.jpg"),
    boss!("Lvl 100 Dark Angel Olivia", 100, En, "https://pbs.twimg.com/media/CfqXOjEUMAAXuK2.jpg"),
    boss!("Lvl 100 Garuda", 100, En, "https://pbs.twimg.com/media/CkVbjdpUYAAmnvb.jpg"),
    boss!("Lvl 100 Grand Order", 100, En, "https://pbs.twimg.com/media/CfqaAYfUUAQqgpv.jpg"),
    boss!("Lvl 100 Grani", 100, En, "https://pbs.twimg.com/media/CqwDYIbVMAE0VUR.jpg"),
    boss!("Lvl 100 Leviathan Omega", 100, En, "https://pbs.twimg.com/media/CfqZfExVIAA4YgF.jpg"),
    boss!("Lvl 100 Lich", 100, En, "https://pbs.twimg.com/media/CqwDaPAVYAQAYzq.jpg"),
    boss!("Lvl 100 Luminiera Omega", 100, En, "https://pbs.twimg.com/media/CfqZvtlVIAAgBeF.jpg"),
    boss!("Lvl 100 Macula Marius", 100, En, "https://pbs.twimg.com/media/CfqXAC1UIAAeGl-.jpg"),
    boss!("Lvl 100 Medusa", 100, En, "https://pbs.twimg.com/media/CfqXEh_UsAEb9dw.jpg"),
    boss!("Lvl 100 Nezha", 100, En, "https://pbs.twimg.com/media/CfqW0bzUMAAOJsU.jpg"),
    boss!("Lvl 100 Odin", 100, En, "https://pbs.twimg.com/media/CqwDWGjUIAEeJ4s.jpg"),
    boss!("Lvl 100 Proto Bahamut", 100, En, "https://pbs.twimg.com/media/CfqXYlBUAAQ1mtV.jpg"),
    boss!("Lvl 100 Tiamat Omega Ayr", 100, En, "https://pbs.twimg.com/media/CfqW2SWUEAAMr7S.jpg"),
    boss!("Lvl 100 Twin Elements", 100, En, "https://pbs.twimg.com/media/CfqXaAvUIAEUC8B.jpg"),
    boss!("Lvl 100 Yggdrasil Omega", 100, En, "https://pbs.twimg.com/media/Cfv1i6wUsAAZajc.jpg"),
    boss!("Lvl 120 Apollo", 120, En, "https://pbs.twimg.com/media/CfqZxihUEAEzcG-.jpg"),
    boss!("Lvl 120 Dark Angel Olivia", 120, En, "https://pbs.twimg.com/media/CfqZ3BwVIAEtIgy.jpg"),
    boss!("Lvl 120 Macula Marius", 120, En, "https://pbs.twimg.com/media/CfqZhE0UsAA_JqS.jpg"),
    boss!("Lvl 120 Medusa", 120, En, "https://pbs.twimg.com/media/CfqZlIcVIAAp8e_.jpg"),
    boss!("Lvl 120 Nezha", 120, En, "https://pbs.twimg.com/media/CfqW4BYUEAAeYSR.jpg"),
    boss!("Lvl 120 Twin Elements", 120, En, "https://pbs.twimg.com/media/CfqZQ_pUEAAQFI4.jpg"),
    boss!("Lvl 150 Proto Bahamut", 150, En, "https://pbs.twimg.com/media/CfqZ-YtVAAAt5qd.jpg"),
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const EXPECTED_PAIRS: &[(&'static str, &'static str)] = &[
    // Guild Wars
    ("Lv30 アーフラー", "Lvl 30 Ahura"),
    ("Lv40 アーフラー", "Lvl 40 Ahura"),
    ("Lv40 ゲイザー", "Lvl 40 Ogler"),
    ("Lv40 ヨグ＝ソトース", "Lvl 40 Yog-Sothoth"),
    ("Lv50 ベオウルフ", "Lvl 50 Grendel"),
    ("Lv60 グガランナ", "Lvl 60 Gugalanna"),
    ("Lv60 ベオウルフ", "Lvl 60 Grendel"),
    ("Lv60 マヒシャ", "Lvl 60 Mahisha"),
    ("Lv75 エメラルドホーン", "Lvl 75 Viridian Horn"),
    ("Lv75 スーペルヒガンテ", "Lvl 75 Supergigante"),

    // Standard
    ("Lv50 セレスト", "Lvl 50 Celeste"),
    ("Lv50 ティアマト", "Lvl 50 Tiamat"),
    ("Lv50 ティアマト・マグナ", "Lvl 50 Tiamat Omega"),
    ("Lv50 ユグドラシル", "Lvl 50 Yggdrasil"),
    ("Lv50 リヴァイアサン", "Lvl 50 Leviathan"),
    ("Lv50 ヴェセラゴ", "Lvl 50 Veselago"),
    ("Lv60 ユグドラシル・マグナ", "Lvl 60 Yggdrasil Omega"),
    ("Lv60 リヴァイアサン・マグナ", "Lvl 60 Leviathan Omega"),
    ("Lv70 コロッサス・マグナ", "Lvl 70 Colossus Omega"),
    ("Lv75 シュヴァリエ・マグナ", "Lvl 75 Luminiera Omega"),
    ("Lv75 セレスト・マグナ", "Lvl 75 Celeste Omega"),
    ("Lv100 Dエンジェル・オリヴィエ", "Lvl 100 Dark Angel Olivia"),
    ("Lv100 アテナ", "Lvl 100 Athena"),
    ("Lv100 アポロン", "Lvl 100 Apollo"),
    ("Lv100 オーディン", "Lvl 100 Odin"),
    ("Lv100 ガルーダ", "Lvl 100 Garuda"),
    ("Lv100 グラニ", "Lvl 100 Grani"),
    ("Lv100 コロッサス・マグナ", "Lvl 100 Colossus Omega"),
    ("Lv100 シュヴァリエ・マグナ", "Lvl 100 Luminiera Omega"),
    ("Lv100 ジ・オーダー・グランデ", "Lvl 100 Grand Order"),
    ("Lv100 セレスト・マグナ", "Lvl 100 Celeste Omega"),
    ("Lv100 ティアマト・マグナ＝エア", "Lvl 100 Tiamat Omega Ayr"),
    ("Lv100 ナタク", "Lvl 100 Nezha"),
    ("Lv100 バアル", "Lvl 100 Baal"),
    ("Lv100 フラム＝グラス", "Lvl 100 Twin Elements"),
    ("Lv100 プロトバハムート", "Lvl 100 Proto Bahamut"),
    ("Lv100 マキュラ・マリウス", "Lvl 100 Macula Marius"),
    ("Lv100 メドゥーサ", "Lvl 100 Medusa"),
    ("Lv100 ユグドラシル・マグナ", "Lvl 100 Yggdrasil Omega"),
    ("Lv100 リッチ", "Lvl 100 Lich"),
    ("Lv100 リヴァイアサン・マグナ", "Lvl 100 Leviathan Omega"),
    ("Lv120 Dエンジェル・オリヴィエ", "Lvl 120 Dark Angel Olivia"),
    ("Lv120 アポロン", "Lvl 120 Apollo"),
    ("Lv120 ナタク", "Lvl 120 Nezha"),
    ("Lv120 フラム＝グラス", "Lvl 120 Twin Elements"),
    ("Lv120 マキュラ・マリウス", "Lvl 120 Macula Marius"),
    ("Lv150 プロトバハムート", "Lvl 150 Proto Bahamut"),
];
