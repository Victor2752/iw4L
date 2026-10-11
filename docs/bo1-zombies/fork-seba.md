# O fork do seba e a `kino`

A `kino` saiu do `seba/main` (`seba2390/iw4L`) e o seba continua trabalhando
lá. Integramos o trabalho dele em 2026-10-11 em dois merges:

- `9a2eee2`: até o `e60be09`, 153 commits;
- `88fdedd`: o PR #53.

A seção "Integrar de novo" abaixo explica como repetir.

## O que veio dele

| área | o que é |
|---|---|
| **movimento do BO1** | crate `movement_t5` atrás de `game_api::PlayerMovement`: andar, pulo, sprint, agachar, deitar, dive to prone, lean e colisão. O que falta está nos itens `t5.movement.*` do ledger (mantle, eventos de passos, água). |
| **armas do BO1** | `movement_t5::weapon`: troca, tiro (rajada, contínuo), rechamber, recarga, melee, faca, offhand, ADS e spread. As armas são lidas pela definição do BO1 (`fastfile_t5::weapon_def`). |
| **perks** | Speed Cola, Double Tap (`perk_weapRateMultiplier` 0.75), sprint e fôlego, nas regras do BO1. |
| **tiro, melee e explosão** | `game_t5/src/fire.rs`, `radius.rs` e `missile.rs`: bala do servidor com tabela de hit location e penetração, melee, `RadiusDamage` e foguetes. |
| **zonas do zombies** | efeitos de impacto, efeitos e materiais de arma das zonas zombie (Ray Gun, garrafa de perk). |
| **motor (upstream)** | Bevy 0.20, escopos de partida (`frame::InMatch`, `MatchScope`), progressão local do MW2 e renderização em pipeline no macOS. |
| **BO2 e BO3** | scripts, mundo e builtins (fora do nosso foco). |

## O que foi decidido no merge

- O `T5OnIw4Rules` (`session/src/games.rs`) deixou de emprestar movimento e
  armas do MW2. Continua emprestando o last stand, o HUD de código, o layout
  de menu e o `onlinegame 1`. O last stand do MW2 funciona por cima do
  movimento do BO1, porque o `pm_type` dele vira o `MoveType::LastStand` do
  BO1.
- O nosso Double Tap na máquina de armas do MW2 saiu
  (`ModeRules::double_tap_rate`, `PERK1_RATEOFFIRE`). Ficou o do seba.
- A nossa leitura da tabela de dano por região saiu e ficou a dele. No
  `actors.rs`, um tiro que a regra do BO1 já multiplicou não é
  multiplicado de novo.
- O nosso `SetMaxHealth` do zombies (vida e vida máxima) continua valendo
  por cima do genérico dele.
- O `add_scene_brush_surfaces` foi para o set `frame::InMatch`, porque o
  `HostGfxScene` agora só existe na partida. Sem isso o jogo entrava em
  pânico.

## Bugs que o merge revelou (e corrigimos)

Com as armas do BO1, o estado da arma vive nos bytes do jogo, mas vários
lugares ainda liam o `ps.weaponstate_primary` do MW2, parado em "subindo":

- os builtins `isFiring`, `isMeleeing`, `isThrowingGrenade` e
  `isSwitchingWeapons` (`sim/src/script/host/natives/t5.rs`);
- os notifies `weapon_switch_started` e `weapon_change_complete`
  (`sim/src/script/host/weapons.rs::notify_weapon_changes`).

Por causa disso, a máquina de perk recusava toda compra e a garrafa nunca
terminava: o Quick Revive solo não dava a vida e o down virava fim de jogo.
Agora eles perguntam ao jogo (`PlayerMovement::weapon_activity`, com a
regra de `PlayerCmd_IsSwitchingWeapons` e afins). Esse bug também existe no
fork do seba.

Os números de estado de arma da decompilação (`bgame/bg_weapons.h`, código
`game_mp`) não batem com os do executável SP que o seba leu: o SP tem
`RECHAMBER_END` = 8 e desloca o resto. Por isso as regras seguem os
**nomes** dos estados na numeração do `movement_t5`.

## Integrar de novo

```bash
git fetch seba
git log --oneline kino..seba/main
git merge-tree --write-tree --name-only kino seba/main
git merge --no-ff --no-commit seba/main
```

1. O `merge-tree` mostra os conflitos antes de mexer na árvore.
2. Em cada conflito, decida pela tabela acima: o que é regra do BO1 que ele
   portou fica com ele; menus, HUD, brushes, modelos estáticos e pausa
   ficam com a gente.
3. Antes de commitar:
   - `cargo build --profile play -p launcher`;
   - `cargo xtask boundary --enforce` (`--update` se só baixou contagem);
   - `make publish-check`;
   - o roteiro curto de [testes](testes.md#depois-de-um-merge).
4. O `PROTOCOL_VERSION` muda dos dois lados: use um número acima dos dois.
5. O `allow.txt` e o `docs/fidelity/t5.md` sempre conflitam. Junte as duas
   listas.
