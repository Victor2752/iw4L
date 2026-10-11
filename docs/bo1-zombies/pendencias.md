# Pendências, em ordem

Cada item, ao fechar, sai daqui, entra em [`implementado.md`](implementado.md)
e fecha a entrada no `docs/fidelity/t5.md`. Antes de começar um item, leia a
função do BO1 na [decompilação](decompilacao.md#fluxo-de-trabalho). O teste
vem depois, para confirmar.

1. **Trazer o trabalho do seba** ([fork-seba.md](fork-seba.md)). Ele já
   portou o movimento, a máquina de armas, os perks, o tiro, o melee e as
   explosões do BO1. São os itens que vinham a seguir nesta lista. Fazer o
   merge do `seba/main` na `kino` e ficar com o dele nessas áreas.
2. **Modelos de outra zona** (58 dos 3326 modelos estáticos da Kino: ursinhos,
   porta do lobby). Resolver a referência pelo nome nas zonas do zombies,
   como o DB do BO1 faz, e dar colisão a eles.
3. **Teleporte**: testar o link (pad → mainframe), a troca dos cabos
   (`link_cable_off`/`_on`), a viagem e a volta. Sem CSC, os efeitos dele não
   aparecem (item 6). Verificar se `exploder()` (GSC + `createfx`) toca algo;
   o parser `createfx` do IW4L não lê exploders.
4. **Vision set** (imagem "apagada"). `assets/src/lane/t5.rs` descarta as
   `.vision`. O seba espera um trace de D3D9, mas a decompilação já tem a
   cadeia em `gfx_d3d/rb_postfx.cpp::RB_BloomLDR`:
   - bloom: `bloom_downsample_ldr` → `resampleFinal` → `bloom_downsample_convolution`
     → `bloom_remap` → blur X/Y → streak → `bloomSmoothA`;
   - `bloom_apply_ldr` (curva Hable, W=8);
   - `postfx_color` (`POSTFX_CONTROL0..A`: matrizes de temperatura, matiz e
     saturação, contraste, bleach e as bandas mid).

   O IW4L já traduz os shaders nativos de pós do T5; falta montar a cadeia e
   as constantes.
5. **Last stand do BO1** (`game_sp/g_sp_client.cpp`). Hoje usa o pm type e a
   altura de visão do MW2. O seba também não tem (no dele o jogador morre).
6. **Client scripts (CSC)**: nenhum roda. Isso tira:
   - o brilho dos olhos dos zumbis;
   - os efeitos de tábuas e risers;
   - a música (`clientsyssetstate`);
   - as luzes da energia;
   - os efeitos do teleporte;
   - a luz do projetor depois que a tela desce (`clientnotify("sip")`).
7. **Restos dos menus** ([menus.md](menus.md)):
   - a tecla `pause` (`CL_Pause_f`);
   - as animações de item (`ui_animate`) e o blur;
   - a retícula sobre o menu;
   - o layout de texto do BO1 (`t5.hud.menu_layout`).
8. **Sons**: `uin_transition_zombie_theater` (tela de loading) e os passos
   `fly_step_*`/`fly_gear_*`. Os passos devem vir com o movimento do seba.
