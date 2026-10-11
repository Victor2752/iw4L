# Arquitetura: onde cada regra do BO1 mora

## Camadas

Cada crate declara uma camada em `[package.metadata.iw4l]` no `Cargo.toml`:

- **format** (`fastfile_t5`, `fastfile_iw4`…): lê as zonas `.ff` de um jogo.
  `fastfile_t5/src/load/*.rs` percorre os assets do BO1; `size.rs` guarda os
  tamanhos e offsets das structs.
- **game** (`game_t5`, `game_iw4`, `weapon_iw4`, `movement_iw4`…): as regras de
  um jogo. Os crates `*_iw4` são o MW2 (movimento, armas, HUD, playerstate).
- **neutral** (`game_api`, `gsc`, `menu_expr`…): o que serve a todos os jogos.
- **mixed** (`sim`, `hud`, `render_*`, `assets`, `asset_*`…): a maior parte do
  motor. Hoje ainda mistura jogos; `cargo xtask boundary` mede essa dívida.
- **session** (`session`, `launcher`, `console`, `ui`): monta a partida.

## A fronteira de jogo

Uma partida roda as regras do jogo do mapa.

- `crates/game_api/src/lib.rs` define o que um jogo fornece: `GameScripts`
  (scripts e dvars), `GameModes` → `ModeRules` (movimento, armas, last stand,
  HUD, Double Tap…), `GameMenus` (fonte e layout de menu) e `GameVision`.
- `crates/game_t5/src/lib.rs` responde pelo BO1. Uma regra conhecida é
  `Rule::Known(...)`. Uma desconhecida é `Rule::Unknown(unknown!("t5.area.regra", ...))`,
  e o motor registra `game gap t5.…` no log em vez de usar a regra de outro jogo.
  Todo id `t5.*` precisa estar em `docs/fidelity/t5.md`.
- `crates/session/src/games.rs` é o único lugar que escolhe o jogo por
  `FamilyId` (`scripts`, `modes`, `menus`, `natives`, `vision`).

## O empréstimo do MW2 (provisório)

O fork do seba desligou tudo que o BO1 pegava do MW2, porque as regras do BO1
eram desconhecidas. Para a Kino ficar jogável, `session/src/games.rs` tem o
wrapper `T5OnIw4Rules`. Ele responde pelo BO1, mas pega do MW2:

- movimento (`movement_iw4`), máquina de armas (`weapon_iw4`), last stand;
- HUD de código (`hud.code_hud`) e layout de menu (`game_iw4::MenuLayout`);
- `config_defaults` (`onlinegame 1`, para o game over solo sair por `ExitLevel`).

Cada item desse wrapper deve virar regra do BO1, lida da
[decompilação](decompilacao.md). Quando virar, a resposta sai do wrapper e
passa a vir de `game_t5`.

O seba já trocou movimento e armas pelas regras do BO1 (`movement_t5`,
`MovementRules::Game`, `WeaponRules::Game`). Depois do merge
([fork-seba](fork-seba.md)), esses dois saem do wrapper.

## Regra de ouro: regra do jogo certo

O que fizemos com fontes é o modelo para o resto: a regra fica no crate do
jogo (`game_t5::GameMenus::font` = `UI_GetFontHandle` do BO1), o motor
pergunta ao jogo da partida e nunca cai para outro jogo em silêncio.
Valores que o host e o cliente precisam ver iguais (o cliente prediz
movimento e armas com o mesmo `sim`) vão em `ModeRules`, que é conteúdo da
partida, e não em dvars de script (o cliente não tem a VM de script).

## Fluxo de uma partida BO1

1. **Carga** (`assets/src/session_load/`): `match_walk.rs` abre o mapa;
   `common_walks.rs::walk_zombie_commons` abre `code_post_gfx`, `common`,
   `common_zombie` e o patch (scripts, armas, animações, modelos) e o catálogo de
   menus do BO1 (`asset_game::load_t5_menu_catalog`).
2. **Scripts**: os GSC da Kino rodam na VM do IW4L (`crates/gsc`, `sim/src/script`).
   Os builtins do BO1 ficam em `sim/src/script/host/natives/t5*.rs`, atores em
   `host/actors.rs`, a IA em `host/actor_*.rs`.
3. **Simulação** (`crates/sim`): `step.rs` roda o pmove e a máquina de armas do
   MW2 (liberados por `ModeRules.movement/weapons`); `combat.rs` monta o
   `WeaponCmd`; `script_player.rs` faz perks, vida e last stand.
4. **Cliente**: `hud/src/t5_hud.rs` desenha os menus de HUD do BO1;
   `hud_elems.rs` desenha os textos de script; `render_frontend` desenha o mundo.
   - Modelos estáticos do mapa: `asset_world/src/model_mesh.rs::build_t5_static_model_instances`
     lê as instâncias e `assets/src/lane/helpers.rs::link_model_placements`
     liga cada uma a uma malha.
   - Brush models (portas, tábuas, tela): ver [brushes.md](brushes.md).

## Dívida de fronteira

`xtask/boundary/allow.txt` lista cada uso de um crate de jogo fora do lugar.
A regra é que ela **só diminui**. Quando um passo precisa de um uso novo (o
caso dos empréstimos do MW2), registre a linha nesse arquivo num commit que
explique o motivo, como `265705c` e `23ab175`. Rode
`cargo xtask boundary --enforce` antes de commitar.
