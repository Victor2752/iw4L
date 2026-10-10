# O que o fork já implementou

Cada item traz a regra do BO1 de origem, os arquivos e o commit na `kino`.

## Partida jogável com o MW2 emprestado — `e7324bb`

`session/src/games.rs::T5OnIw4Rules` (ver [arquitetura](arquitetura.md#o-empréstimo-do-mw2-provisório)).
Liga movimento, armas, last stand e HUD de código do MW2 para o BO1 e usa
`onlinegame 1` do MW2, para o game over solo sair por `ExitLevel`.

## Dano por região (headshot) — `a5a0d4c`

- Regra: `game/bullet.cpp`. Um tiro em ator é multiplicado pelo
  `locationDamageMultipliers` da arma na região do osso atingido.
- Leitura: `fastfile_t5/src/size.rs::WEAPON_DEF_LOCATION_DAMAGE_OFF` e
  `asset_game/src/weapon_catalog/capture_t5.rs::read_location_damage_t5`
  (19 floats; o 20º, escudo, fica 1.0).
- Aplicação: `sim/src/script/host/actors.rs::location_scaled`. Só tiros em
  osso; explosão e faca não têm região.

## Fontes e texto de script — `3ffaefc`

- `game_t5::GameMenus::font` = `UI_GetFontHandle`: os enums fixos dão a
  fonte; os outros escolhem pela escala contra `ui_smallFont` 0.25 e
  `ui_bigFont` 0.4 (o 4 pega a bold).
- `hud/src/hud_elems.rs::t5_hudelem_font`: tabela de `GetHudElemInfo`.
- `t5_snapshot_font_scale`: o `fontScale` vai no snapshot em 6 bits
  (`round((fs-1)*10) & 63`, lido como `*0.1 + 1`). Sem isso o texto dos
  popups "+10" saía 4× maior.

## Coluna de pontos — `108b430`

- Owner draw 288 (`CG_DrawCompetitiveModeScores` / `CG_DrawPlayerScoreBar`)
  em `hud/src/t5_hud.rs`, com contagem animada.
- `CGAME_DVARS`: altura de linha, fonte e as cores do `coop_arcademode.cfg`.
- `hud/src/weaponbar.rs` passa os placares (`scores`) e o cliente local.

## Tábuas, portas e entulho — `33e6dcf`

- O mundo T5 guarda brush models em `GfxBrushModel` (`world_t5.rs::decode_brush_models`).
- O backend ignora a transformação de itens do mundo, então a pose autorada é
  **assada nos vértices** (`render_frontend/src/prepare/scene/world.rs::bake_t5_script_brush_poses`).
- `render_frontend/src/adapters/anim/dyn_ent_brush.rs::add_scene_brush_surfaces`
  só desenha a brush enquanto ela está na pose autorada. Por isso uma porta
  aberta **some** em vez de girar ([pendência](pendencias.md)).

## Itens de HUD com `showDvar`/`hideDvar` — `835f199`

`hud/src/chrome.rs::shown_via_dvar` = `Item_EnableShowViaDvar`. Corrigiu o
ícone de granada fixo no centro (menu `deadscreen`, `dvarTest ui_grenade_death`).

## Perks — `5f73f1d`, `71efa31`

- **Double Tap**: `Perks_RegisterDvars` registra `perk_weapRateMultiplier`
  0.75. Com `specialty_rof`, `PM_Weapon_WeaponTimeAdjust` roda o tempo de
  tiro e de rechamber de **toda** arma a `msec / 0.75`, arredondado (8 ms → 11 ms).
  - Valor: `ModeRules::double_tap_rate` (`game_t5` = `Some(0.75)`, MW2/T6 = `None`).
  - Bit: `weapon_iw4::PERK1_RATEOFFIRE` (0x100 em `perks[1]`, o bit do BO1),
    setado em `sim/src/script_player.rs::perk_bits`.
  - Escala: `weapon_iw4/src/tick.rs::weapon_time_adjust`, estados 6 e 7.
- **Speed Cola**: o fast reload do MW2 já usa 0.5, o mesmo padrão do BO1.
- **Juggernog / revive**: o `SetMaxHealth` do zombies define vida **e** vida
  máxima (`sim/src/script/host/natives/t5_zombie.rs`). O Jugger enche até 250;
  o revive volta com a vida cheia.
- **Quick Revive solo**: funciona pelos scripts; foi testado comprando, caindo
  e levantando sozinho.

## Registro da fronteira — `23ab175`

Os usos novos de crates `*_iw4` que esses commits trouxeram estão em
`xtask/boundary/allow.txt`.

## Menus do BO1 (pausa e opções)

Ver [menus.md](menus.md) para o funcionamento.
- ESC abre o `pausedmenu` do BO1 (RESUME CARNAGE, SETTINGS, RESTART LEVEL,
  QUIT). SETTINGS abre as opções do BO1 (Graphics, Sound, Voice Chat, Game,
  Controls).
- Eventos, scripts if/else, `showMenu`/`hideMenu`, variáveis locais, todas as
  funções de expressão pelo `functions.txt` do jogo, as conversões do BO1 e o
  slider em blocos.
- Arquivos: `asset_game/src/menu_catalog_t5.rs` (leitura),
  `hud/src/menus/{mod,runtime,script,host}.rs` (execução e pintura),
  `game_t5/src/menu_expression.rs` (expressões), `game_api` (`ingame_menu`,
  `slider`), `session/src/match_apply.rs` (dvars de motor publicados).

## FPS e sensibilidade numérica (adições do IW4L)

`ui/src/fps.rs` (contador), `ui/src/game_options.rs` (linhas novas nos menus
do BO1), `frame/src/settings.rs` (`show_fps`) e
`console/src/user_settings.rs` (`sensitivity` e `cg_drawFPS`, nos dois sentidos).

## Pausa no solo

`frame::MatchPause`, ligado por `hud/src/menus/mod.rs` quando o ESC abre um
menu que o jogo manda pausar (`InGameMenu::Named { pauses_offline }`) e a
partida é offline; lido em `net/src/authority/runtime.rs::authority_should_tick`
e em `net/src/client/runtime.rs` (`predict_local_move`, `advance_cg_frame_clock`).
Regra: `CL_PauseGame`, `SV_CheckPaused` e `CL_Frame` do BO1.

