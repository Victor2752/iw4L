# Menus do BO1

## De onde vêm

Os menus do BO1 estão nas zonas `code_post_gfx.ff` e `patch.ff`; as fontes vêm
do `en_code_post_gfx.ff` e os textos `@PATCH_*` do `en_patch.ff`.
`assets/src/session_load/common_walks.rs` carrega esse catálogo para toda
partida T5 (`SessionHudMenus`) com `asset_game::load_t5_menu_catalog`
(`asset_game/src/menu_catalog_t5.rs`). São 189 menus. Dentro da partida o BO1
zombies usa as listas `ui/ingame.txt` e `ui/ingame_options.txt`
(`asset_game::T5_INGAME_MENU_LISTS`), além das listas de HUD.

## Como o motor do BO1 funciona (e o IW4L reproduz)

- **ESC** (`UI_SetActiveMenu(UIMENU_INGAME)`): no zombies abre o
  `pausedmenu`, desde que o `ingameoptions` esteja carregado. É a regra
  `GameMenus::ingame_menu` (`game_t5`); o MW2 abre o menu do `g_scriptMainMenu`.
- **Eventos**: cada menu e cada item têm uma lista de `GenericEventHandler`
  (nome + script). Os nomes são `onOpen`, `onClose`, `onESC`, `action`,
  `onAccept`, `onFocus` e `leaveFocus`, além de `mouseEnter`/`mouseExit` e
  `onKey` (por tecla).
- **Scripts de evento** (`GenericEventScript`): lista plana de elos com ação
  em texto, condição, tipo (else = 3), bloco, construção e pré-requisitos.
  `Item_RunSingleEventScript` é portado em `hud/src/menus/runtime.rs::Runner::t5_link`.
- **Comandos**: os de texto já existentes, mais os do BO1:
  - `showMenu`/`hideMenu` mostram um menu inteiro **embaixo** da pilha;
  - `setLocalVar*` define as variáveis locais do menu (`ui_highlight`,
    `ui_choicegroup`), que acendem as barras de destaque;
  - `nosavehide`/`saveAvailableHide`: o IW4L não guarda saves, então "SAVE
    AND QUIT" some e "QUIT" aparece.
- **Expressões** (`game_t5/src/menu_expression.rs`): o índice de cada função
  é a linha dela no `expressions/functions.txt` do próprio jogo, **+24**. As
  conversões seguem os `RPN_OP_*` do BO1. As consultas ao host (variável
  local, menu aberto, foco, largura de texto, em jogo, servidor) passam pelo
  trait `menu_expr::GameQueries` e chegam no `hud/src/menus/host.rs::MenuHost`.
- **Dvars de motor**: os que o jogo define (`GameScripts::engine_dvars`, por
  exemplo `zombiemode 1` e `ui_multiplayer 0`) são publicados aos clientes
  (`sim::SimWorld::publish_gsc_dvar`). É assim que os menus escolhem as linhas
  do zombies.
- **Slider**: `Item_Slider_Paint`, com 10 blocos laranja na coluna de valor e
  o thumb `ui_arrow_right` girado (`GameMenus::slider` → `SliderStyle::Blocks`,
  `hud/src/menus/mod.rs::slider_blocks`).

## Adições do IW4L (não existem no BO1)

Ficam em `crates/ui/src/game_options.rs`, que clona as linhas do BO1 quando o
catálogo da partida é instalado:

- **Controls → Look → MOUSE SENSITIVITY (VALUE)**: campo decimal abaixo do
  slider (a barra continua). Enter começa vazio, digite o número, Enter grava
  `sensitivity`.
- **Settings → Graphics → SHOW FPS**: Sim/Não no `cg_drawFPS`. A linha ocupa o
  lugar do FOV, que o BO1 só mostra no multiplayer. O contador fica no canto
  superior direito (`crates/ui/src/fps.rs`).

Os dois valores são salvos no `settings.cfg` (`sensitivity=`, `show_fps=`),
com a ponte em `console/src/user_settings.rs::native_menu_settings`.

Cada linha de opções do BO1 tem 5 itens: barra, barra de destaque, rótulo,
rótulo destacado e controle. `clone_row` copia esse conjunto, renumera o
`ui_highlight` e monta a expressão de destaque no formato `t5`.

## O que falta

Ver [pendências](pendencias.md): pausar no solo, animações dos itens
(`ui_animate`) e blur, a retícula aparecendo sobre o menu e o layout de texto
do BO1, que ainda é o do MW2.

## Como inspecionar e testar

- Console: `togglemenu` (o ESC), `openmenu <menu>`, `closemenu <menu>`,
  `menukey up|down|left|right|enter|escape` e `menutext <texto>`.
- Dump do catálogo em JSON: [`testes.md`](testes.md#dump-dos-menus-do-bo1).
