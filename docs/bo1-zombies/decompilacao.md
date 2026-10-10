# A decompilação do BO1

## O que é

`https://github.com/MisaDev4/bo1-zombies-decompiled` é o motor SP/zombies do
Black Ops 1 decompilado em C++ e anotado (cerca de 2.600 arquivos, **sem
licença**). Muitas funções têm comentários com evidências e medições. É a
fonte das regras do BO1 que o fork implementa.

## Onde fica

Clone **fora** do repositório do IW4L:

```bash
git clone --depth 1 https://github.com/MisaDev4/bo1-zombies-decompiled.git /Users/victor2752/Projetos/bo1-zombies-decompiled
```

Para atualizar: `git -C /Users/victor2752/Projetos/bo1-zombies-decompiled pull`.

## Como usar (regras)

- Só leitura: grep e leitura. Não compile nem rode nada dele.
- Leia a regra e **reimplemente em Rust**, no estilo do crate onde ela entra.
  Não copie código.
- **Nunca** traga endereços do executável (números hex longos) nem nomes
  `FUN_`/`DAT_` para código, comentários ou docs. O `make publish-check` recusa.
- Nomes de função e de dvar do motor podem ser citados em comentário
  (`PM_Weapon_WeaponTimeAdjust`, `perk_weapRateMultiplier`); o repo já faz isso.
- Uma regra lida da decompilação fecha a entrada correspondente em
  `docs/fidelity/t5.md`, citando a função de origem.

## Mapa útil (`src/`)

| área | arquivos |
|---|---|
| movimento | `bgame/bg_pmove.cpp`, `bg_slidemove.cpp`, `bg_jump.cpp`, `bg_mantle.cpp` |
| armas e perks | `bgame/bg_weapons.cpp` (máquina de estados, `PM_Weapon_WeaponTimeAdjust`), `bg_perks.cpp` (`Perks_RegisterDvars`), `bg_weapons.h` (enum de estados) |
| dano | `game/bullet.cpp` (multiplicador por região em atores) |
| IA | `game/actor_*.cpp`, `actor_zombie_exposed.cpp` |
| jogador SP | `game_sp/g_sp_client.cpp` (last stand), `g_scr_sp_players_x1.cpp` (`setmaxhealth`), `g_scr_sp_entity.cpp` |
| HUD | `cgame/cg_sp_hud.cpp` (pontos), `cg_hudelem.cpp` (texto de script), `cg_visionsets.cpp` |
| render | `gfx_d3d/rb_postfx.cpp` (bloom, film, cor), `r_material.cpp`, `r_dpvs.cpp` (brush models), `r_gfx.h` (structs) |
| menus | `ui/ui_shared.h` (structs `menuDef_t`, `itemDef_s`, eventos), `ui/ui_shared.cpp` (motor), `ui/ui_main.cpp` (fontes), `ui_mp/ui_main_mp.cpp` (`UI_SetActiveMenu`: ESC → `pausedmenu`) |
| expressões de menu | `universal/com_expressions_eval.cpp` |
| rede | `qcommon/sv_msg_write.cpp` (codificação de campos do snapshot) |
| client script (CSC) | `clientscript/`, `cgame/cg_scr_sp_client.cpp` |

## Método que funcionou

1. Achar a função pelo nome do dvar, do owner draw ou do menu (`grep -rn`).
2. Ler a regra e os valores padrão (`_Dvar_Register*`).
3. Conferir nos scripts GSC da Kino que valores os scripts passam
   ([dump em `testes.md`](testes.md#dump-dos-scripts-gsc)).
4. Implementar no crate do jogo, rodar a Kino e conferir com screenshot e log.
