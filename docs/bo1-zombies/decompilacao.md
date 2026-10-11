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

## Fluxo de trabalho

A decompilação é a base, e o teste só confirma. Testar no escuro gasta tempo:
nas portas foram várias rodadas de screenshot para ver um giro de 1 s,
enquanto `R_AddBModelSurfacesCamera` e o `door_activate` do GSC já diziam o
que esperar. Ler primeiro também acha bugs que nenhum teste mostra: os
modelos estáticos da Kino sumiam porque a struct `GfxStaticModelDrawInst` tem
uma entrada por modelo, e o leitor pulava algumas.

1. **Ler a regra.** Ache a função pelo nome do dvar, owner draw, menu, builtin
   ou struct (`grep -rn`). Leia a regra, os valores padrão (`_Dvar_Register*`)
   e a struct (`r_gfx.h`, `r_bsp.h`, `ui_shared.h`…).
2. **Ler o script.** Confira no GSC da Kino o que o script chama e com que
   valores ([dump](testes.md#dump-dos-scripts-gsc)).
3. **Ler o log da carga.** Antes de rodar o jogo, procure no relatório da
   carga números que não batem: `world report: …`, `refused`, `unresolved`,
   `gap`. Um "0/3326" ou um "decoded X but count is Y" já mostra o defeito.
4. **Implementar** no crate do jogo (ou no leitor do formato), citando a
   função do BO1 no comentário.
5. **Confirmar com um teste curto**: uma rodada, com `score`, `move` e
   `god` para chegar direto ao ponto, e um ou dois screenshots. Para coisas
   rápidas (um giro, uma animação), um log temporário vale mais que fotos
   (cada screenshot trava a fila do console por cerca de 1 s).

## Mapa extra

| assunto | onde |
|---|---|
| modelos estáticos | `gfx_d3d/r_bsp.h` (`GfxStaticModelDrawInst`, `GfxWorldDpvsStatic`), `gfx_d3d/r_add_staticmodel.h` |
| brush models | `gfx_d3d/r_scene.cpp` (`R_AddBModelSurfacesCamera`), `gfx_d3d/r_dpvs.cpp` |
| pausa | `ui_mp/ui_main_mp.cpp` e `ui/ui_main.cpp` (`UI_SetActiveMenu`, que chama `CL_PauseGame`), `client_mp/cl_main_mp.h` |
