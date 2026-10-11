# Build, testes e inspeção

## Build e execução

- `.env` define `IW4L_GAMES=/Users/victor2752/Games`, com `iw4/` (MW2) e `t5/`
  (BO1: `zone/common/*.ff`, `zone/English/en_*.ff`). Carregue-o com
  `set -a; . ./.env; set +a`.
- Build: `cargo build --profile play -p launcher` (o incremental leva ~10–30 s).
- Jogar: pelo launcher (Black Ops → Zombies → Kino) ou
  `IW4L_GAMETYPE=zom ./target/play/iw4l map t5:zombie_theater`.

## Rodar a Kino por script

```bash
IW4L_SOUND=off IW4L_GAMETYPE=zom ./target/play/iw4l map t5:zombie_theater --cmds 'wait world; spawn 0; god; wait 6s; move 531 -1205 80 270 10; hold +activate; wait 1s; release +activate; wait 6s; screenshot qr; finish_run'
```

- Screenshots: `iw4l-artifacts/screenshots/<nome>.png`. Logs: `iw4l-artifacts/logs/<id>.log`
  (o mais novo: `ls -t iw4l-artifacts/logs/*.log | head -1`).
- `IW4L_LOG_LEVEL=debug` mostra os acertos em atores (`actor N: hit …`) e os pontos (`player 0: score N`).
- Posições úteis: [tabela abaixo](#posições-úteis-na-kino). As outras estão
  no `entities.txt` do dump GSC (o `origin` do trigger ou do modelo).
- Verbos: [`../RUN.md`](../RUN.md). `look actor` mira no zumbi mais próximo;
  `damage 300` derruba o jogador; `god` liga invencibilidade.
- Cuidados:
  - se alguém mexer na janela durante o teste, os `wait`/`look` expiram e até o
    `wait world` pode estourar em 120 s (o log mostra `spawned=false`);
    repita a rodada;
  - o macOS não tem `timeout`;
  - use `IW4L_SOUND=off` só em testes automáticos.

## Testes baratos

Cada rodada carrega o mapa (~20 s) e cada screenshot custa tempo e tokens
para ler. Para economizar:

- **Pontos**: `score 5000` em vez de matar zumbis. O `zombie_cheat` dos
  scripts não serve: fica em bloco `/# #/`, que o compilador descarta.
- **Posição**: `move x y z yaw pitch` leva direto ao ponto, e `god` evita
  morrer no caminho.
- **Uma rodada, poucas fotos**: junte o antes e o depois na mesma rodada.
  Cada `screenshot` trava a fila do console por cerca de 1 s (leitura da GPU
  e PNG de 3456×2168), então não dá para fotografar várias etapas de algo
  que dura 1 s. Para movimento rápido, use um log temporário.
- **Log da carga antes do jogo**: `serve t5:zombie_theater --cmds 'wait world; quit'`
  e grep no log (`world report`, `refused`, `unresolved`, `gap`) achou o
  defeito dos modelos estáticos sem nenhuma screenshot.
- **Comece pela decompilação** ([fluxo](decompilacao.md#fluxo-de-trabalho)):
  com a regra lida, um teste basta para confirmar.

## Posições úteis na Kino

| o quê | comando |
|---|---|
| Quick Revive (500 no solo, sem energia), de frente | `move 531 -1320 80 90 5` e `hold +activate` |
| porta do foyer (750), lado do corredor | `move 548 -641 322 0 5` e `hold +activate` |
| a mesma porta vista da sala da caixa | `move 700 -641 322 180 5` |
| alavanca da energia (bastidores) | `move -488 1200 40 90 0` e `hold +activate` |
| palco (cortina e tela) | `move 0 300 140 90 0` |
| pad do teleporte | `move -304 930 90 90 5` |
| cabo do link do teleporte (chão) | `move -141 -190 90 90 50` |

Depois da energia: a cortina leva ~3 s e a tela desce em 6 s.

## Conferir o MW2

Toda mudança deve deixar o MW2 igual. Desde o upstream, a progressão local
trava as classes do perfil (`spawn N` responde "Unlocks at level …"). Por
isso o teste escolhe a classe padrão destacada pelo próprio menu e usa uma
pasta de artefatos à parte, para não mexer no seu perfil:

```bash
IW4L_ARTIFACTS_DIR=/tmp/iw4l-mw IW4L_SOUND=off ./target/play/iw4l map mp_rust --cmds 'wait world; wait 2s; menukey enter; wait 2s; force_match_start; wait 1s; hold +attack; wait 2s; release +attack; screenshot mw; finish_run'
```

## Depois de um merge

Uma rodada cobre o essencial do zombies. O `god` vem depois do jogador
estar vivo (`wait 3s`); antes disso ele responde "not Alive".

```bash
IW4L_SOUND=off IW4L_LOG_LEVEL=debug IW4L_GAMETYPE=zom ./target/play/iw4l map t5:zombie_theater --cmds "wait world; spawn 0; wait 3s; god; score 5000; hold +forward; wait 1s; release +forward; showpos; hold +attack; wait 1s; release +attack; move 548 -641 322 0 5; hold +activate; wait 300ms; release +activate; move 531 -1320 80 90 5; wait 500ms; hold +activate; wait 1s; release +activate; wait 6s; god; wait 500ms; damage 300; wait 14s; togglemenu; wait 1s; screenshot pos_merge; finish_run"
```

No log (`grep -E "score [0-9]+|revive_up|game_over"`), espere:

- 5000 → 4250 (porta) → 3750 (Quick Revive);
- o down e o `revive_up`, sem `mus_zombie_game_over`.

Na screenshot, o menu de pausa do BO1.

## Dump dos scripts GSC

Os GSC vêm legíveis nas zonas. Adicione um hook **temporário**:

- em `assets/src/script_sources.rs`, um método que escreve as fontes e o `entities.txt`;
- em `session/src/match_apply.rs`, a chamada logo após montar `Sources`, quando `TMP_GSC_DUMP` existir.

Depois rode `TMP_GSC_DUMP=<dir> ... serve t5:zombie_theater --cmds 'wait world; quit'`.
São 387 arquivos. Os mais úteis são `maps/_zombiemode*.gsc` (perks, portas,
pontos, spawner), `maps/_laststand.gsc` e `zombie_theater*.gsc`.

## Dump dos menus do BO1

Um hook temporário em `assets/src/session_load/common_walks.rs`, antes de
`commons.hud_menus = Some(hud_menus)`, escreve cada `MenuDef` com
`serde_json::to_string_pretty` em `$TMP_MENU_DUMP/<menu>.json` (e as listas
`hud_menus.list_menus` num `_lists.txt`). Remova o hook antes de commitar.

Para ler uma expressão `t5` de um dump: cada `#N` com N ≥ 24 é a função da
linha N−24 do `expressions/functions.txt` (rawfile do `code_post_gfx.ff`;
descompacte a zona com zlib a partir do byte 12 e procure o nome). `s:` é
texto em hex, `i:` inteiro e `f:` os bits de um float.

## Testar menus

```bash
IW4L_SOUND=off IW4L_GAMETYPE=zom ./target/play/iw4l map t5:zombie_theater --cmds 'wait world; spawn 0; god; wait 5s; togglemenu; wait 1s; menukey down; menukey enter; wait 1s; screenshot menu; finish_run'
```

`IW4L_SETTINGS_PATH=<arquivo>` usa um `settings.cfg` separado, para o teste não
mexer nas configurações de quem joga.

## Antes de commitar

1. `cargo fmt -p <crates tocados>`. Não use `--all`: reformata o repo inteiro.
2. `cargo clippy -p <crates>`. O repo proíbe `unwrap_or_default` em `Option` e `Result`.
3. `make publish-check`: recusa endereços do executável e nomes `FUN_`/`DAT_`.
4. `cargo xtask boundary --enforce` ([dívida de fronteira](arquitetura.md#dívida-de-fronteira)).
5. Atualize `docs/fidelity/t5.md` (feche ou abra a entrada) e estes docs.
6. Commits por assunto, no estilo do histórico ("Draw…", "Scale…"), terminando com
   `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>` quando feitos com o Claude.

O repo não usa testes unitários (só um arquivo tem `#[test]`). Para conferir
uma conta isolada, um teste temporário no crate, removido antes do commit, funciona bem.
