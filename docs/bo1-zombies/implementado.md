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

> O seba portou o tiro do BO1 com as tabelas de hit location e penetração
> (`game_t5/src/fire.rs`). No merge fica a versão dele ([fork-seba](fork-seba.md)).

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

## Tábuas, portas e entulho — `33e6dcf`, `34a6dc4`, `be1c6bb`

Detalhes em [brushes.md](brushes.md).
- O mundo T5 guarda brush models em `GfxBrushModel` (`world_t5.rs::decode_brush_models`).
- O backend ignora a transformação de itens do mundo, então a pose autorada é
  **assada nos vértices** na carga
  (`render_frontend/src/prepare/scene/world.rs::bake_t5_script_brush_poses`).
- `34a6dc4`: uma brush fora da pose autorada é redesenhada **na pose atual**.
  As linhas dela são recalculadas e regravadas no buffer de vértices do
  mundo na GPU, só quando a pose muda (`BakedBrushModel`, `bmodel_live`,
  `write_world_brush_rows`). Resultados na Kino:
  - a porta de 750 gira e fica aberta encostada na parede;
  - a tela de cinema desce depois da cortina.
- `be1c6bb`: o `solid` da entidade vai no snapshot (protocolo 114). Sem ele o
  cliente não reconhecia a brush, desenhava até as escondidas e o cabo
  energizado do teleporte aparecia desde o início.

## Modelos estáticos do mapa — `164e89d`

- Regra: `GfxStaticModelDrawInst` (`gfx_d3d/r_bsp.h`) tem uma entrada por
  modelo (`smodelCount`). O ponteiro do modelo pode apontar para um XModel já
  carregado ou trazer o XModel **inline** (primeiro uso na zona).
- O leitor (`asset_world/src/model_mesh.rs::build_t5_static_model_instances`)
  pulava as 11 entradas inline. A contagem não batia (3315 ≠ 3326) e o motor
  descartava todos os modelos estáticos. A Kino ficava sem poltronas, sem
  teleporte e sem objetos de cena.
- Agora: 3268/3326 resolvidos (234 malhas). Os 58 restantes são modelos de
  outra zona ([pendência](pendencias.md)).

## Cortina e tela de cinema (verificado, sem código novo)

Com a energia ligada, `setupCurtains` toca `o_zombie_theatre_curtain` no
`script_model` da cortina (`animscripted` → `model_anim_scripted`). A cortina
abre, `curtains_move_done` chega, a tela desce (`movez`) e fica em frente ao
palco. A luz do projetor (`clientnotify("sip")`) depende de CSC.

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

> O seba portou a máquina de armas do BO1 com esses perks
> (`movement_t5`). No merge, o Double Tap no `weapon_iw4` e o
> `ModeRules::double_tap_rate` saem ([fork-seba](fork-seba.md)).

## Registro da fronteira — `23ab175`

Os usos novos de crates `*_iw4` que esses commits trouxeram estão em
`xtask/boundary/allow.txt`.

## Menus do BO1 (pausa e opções) — `95eb3cb`

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

## FPS e sensibilidade numérica (adições do IW4L) — `b5dc97f`

`ui/src/fps.rs` (contador), `ui/src/game_options.rs` (linhas novas nos menus
do BO1), `frame/src/settings.rs` (`show_fps`) e
`console/src/user_settings.rs` (`sensitivity` e `cg_drawFPS`, nos dois sentidos).

## Pausa no solo — `ceb03a2`

`frame::MatchPause`, ligado por `hud/src/menus/mod.rs` quando o ESC abre um
menu que o jogo manda pausar (`InGameMenu::Named { pauses_offline }`) e a
partida é offline; lido em `net/src/authority/runtime.rs::authority_should_tick`
e em `net/src/client/runtime.rs` (`predict_local_move`, `advance_cg_frame_clock`).
Regra: `CL_PauseGame`, `SV_CheckPaused` e `CL_Frame` do BO1.

## Verbo `score` para testes — `4251ecd`

`score <pontos>` define os pontos do jogador local (exige cheats, como o
`god`). Os pontos de dev do próprio script (`zombie_cheat`) ficam em blocos
`/# #/`, que o compilador GSC descarta, como no jogo lançado. Arquivos:
`console/src/debug_move.rs`, `sim/src/input.rs` (`ClientAction::DebugScore`),
`sim/src/step.rs`, `net/src/transport/meta_wire.rs` (protocolo 115).
