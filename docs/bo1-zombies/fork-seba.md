# O fork do seba e a `kino`

A `kino` saiu do `seba/main` (`seba2390/iw4L`). Desde então o seba continuou:
na comparação de 2026-10-11 (`git fetch seba`), o `seba/main` tinha **153
commits** que a `kino` não tem. O mais novo era o PR #50 (foguetes), e ainda
havia o PR #51 aberto (granadas, branch `seba/t5/grenades`). Para conferir
de novo:

```bash
git fetch seba && git log --oneline --no-merges $(git merge-base kino seba/main)..seba/main | wc -l
```

Ele lê as regras dos executáveis do BO1 (cópias sem CEG, ver o topo do
ledger dele). Nós lemos as mesmas regras na [decompilação](decompilacao.md).
As duas fontes chegam às mesmas funções (`PM_*`, `Weapon_*`, `UI_*`).

## O que ele tem e nós não

| área | o que fez | por que importa para nós |
|---|---|---|
| **movimento do BO1** | crate `movement_t5` atrás de `game_api::PlayerMovement`: andar, pulo, sprint, agachar, deitar, dive to prone, lean e colisão (o que falta está nos itens `t5.movement.*` do ledger dele) | é a nossa pendência de movimento; ele resolveu antes |
| **armas do BO1** | `movement_t5::weapon`: troca, tiro (rajada, contínuo), rechamber, recarga, melee, faca, offhand, ADS, spread; armas lidas pela definição do BO1 (`fastfile_t5::weapon_def`) | substitui a máquina do MW2 que pegamos emprestada |
| **perks** | Speed Cola, Double Tap (`perk_weapRateMultiplier` 0.75), sprint e fôlego nas regras do BO1 | o nosso Double Tap no `weapon_iw4` fica redundante |
| **tiro, melee e explosão** | `game_t5/src/fire.rs`, `radius.rs`, `missile.rs`: bala do servidor com tabela de hit location e penetração, melee, `RadiusDamage`, foguetes | sobrepõe o nosso dano por região (`a5a0d4c`) |
| **zonas do zombies** | efeitos de impacto do zombies, efeitos e materiais de arma das zonas zombie (Ray Gun, garrafa de perk de três armas) | aqui o log ainda recusa o Ray Gun (`mc/mtl_ray_gun_battery`) |
| **motor** | merge do upstream: Bevy 0.20, escopos de partida, 32 correções e renderização em pipeline no macOS | desempenho no Mac (M1) |

## O que nós temos e ele não

- Fontes do menu e dos textos de script (`UI_GetFontHandle`, `GetHudElemInfo`):
  no ledger dele ainda estão "esperando captura".
- A coluna de pontos (owner draw 288): no dele ainda está "esperando".
- Todas as funções de expressão de menu, pelo `functions.txt`: no dele, só as
  "provadas".
- Os menus de pausa e de opções do BO1, a pausa no solo, o FPS e a
  sensibilidade numérica.
- As brush models: pose autorada, peças escondidas, portas girando e a tela
  descendo ([brushes](brushes.md)).
- Os modelos estáticos do mapa: o leitor dele também descarta os 3326 da
  Kino (`164e89d` corrige).
- O last stand: aqui ele usa o do MW2, emprestado. No dele é
  `t5.match.last_stand` desconhecido, e o jogador morre sem cair.
- O verbo `score` para os testes.

## Integrar: o que esperar

`git merge-tree --write-tree kino seba/main` aponta conflito em 10 arquivos:

- `game_api/src/lib.rs`, `game_iw4/src/lib.rs`, `game_t5/src/lib.rs`:
  `ModeRules` mudou dos dois lados. Ele trocou `movement`/`weapons` para
  enums (`MovementRules::Game`, `WeaponRules::Game`); nós adicionamos
  `double_tap_rate` e as regras de menu.
- `session/src/match_apply.rs`: os dvars de motor publicados (nós) e os
  escopos de partida (upstream).
- `sim/src/script_player.rs`, `sim/src/script/host/actors.rs`,
  `natives/t5_zombie.rs`: perks, dano por região e `setmaxhealth`. Os dois
  lados fizeram a mesma coisa de formas diferentes.
- `hud/src/menus/mod.rs`: os nossos menus do BO1 e o escopo de partida.
- `docs/fidelity/t5.md` e `xtask/boundary/allow.txt`: juntar as duas listas.

Ao resolver:

- **ficar com o dele**: movimento, armas, perks e tiro. Depois, tirar
  movimento e armas do `T5OnIw4Rules` e o `double_tap_rate` do `ModeRules`.
- **ficar com o nosso**: menus, fontes, HUD de pontos, brushes, modelos
  estáticos e pausa.
- **decidir**: o last stand. O nosso é o do MW2 emprestado e precisa ser
  testado com o movimento do BO1 dele. O certo é portar o
  `g_sp_client.cpp` ([pendências](pendencias.md)).

Depois do merge, refazer o roteiro de [testes](testes.md): andar, atirar,
comprar arma, porta e perk, cair e levantar (Quick Revive), menus e pausa.
Conferir também que o MW2 continua igual.

O PROTOCOL_VERSION muda dos dois lados (ele adicionou `round_serial` ao
wire). Use um número acima dos dois.
