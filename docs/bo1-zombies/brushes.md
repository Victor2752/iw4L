# Brush models: portas, tábuas, entulho, tela e cabo

Peças do mapa que os scripts mexem. No `entities.txt` elas são
`script_brushmodel` com `"model" "*N"`, onde N é o índice do brush model (o
`GfxBrushModel` do mundo T5 e o cmodel do clip). Exemplos na Kino:

| peça | entidade | o que o script faz |
|---|---|---|
| porta do foyer (750) | `foyer_top_door`, `*17` e `*38` | `RotateTo(script_angles, 1)` em `_zombiemode_blockers.gsc::door_activate` |
| tábuas das janelas | `script_brushmodel` com `script_noteworthy` 1–6 | `MoveTo`/`RotateTo` ao serem arrancadas, `Hide`/`Show` |
| cabo do teleporte | `teleporter_link_cable_off` `*56`, `_on` `*57` | `_on` fica escondido até o link (`zombie_theater_teleporter.gsc`) |
| tela de cinema | `movie_screen` `*439` | `movez(-466, 6)` depois da cortina (`zombie_theater_movie_screen.gsc`) |
| clip da cortina | `theater_curtains_clip` `*440` | `notsolid()` quando a energia liga |

## O caminho, do servidor até a GPU

1. **Servidor.** `sim/src/carrier.rs::spawn_brush_mover` cria um script mover
   com `state.index = N` e `state.solid = SCRIPT_MOVER_BMODEL_SOLID`
   (0xffffff, a marca de "sou um brush model"). Os scripts mexem nas
   trajetórias `pos`/`apos`; `Hide()` liga `CG_SCRIPT_MOVER_NODRAW` (0x20) no
   `eFlags`.
2. **Wire.** `net/src/transport/meta_wire.rs` (`encode_entity_state` /
   `decode_entity_state`) leva `index`, `solid`, `eFlags` e as trajetórias.
   Até o `be1c6bb` o `solid` não ia: o cliente via 0, não tratava a peça como
   brush e desenhava tudo na pose autorada, inclusive o que estava escondido
   (o cabo energizado aparecia desde o início).
3. **Cena do cliente.**
   `render_frontend/src/prepare/scene/gfx_scene.rs::occupy_script_brush_scene`
   avalia a trajetória no tempo do snapshot. Ela marca a peça escondida
   (NODRAW) ou a admite (`entity_iw4::script_mover_add_bmodel`) e entrega as
   peças a `render_scene::occupy_script_brushes`, que monta as `scene_brushes`
   (índice, origem e quaternion de cada brush na cena). Uma brush sem mover
   fica na pose do mapa.
4. **Carga do mundo.** O backend (`render_backend/src/material_exec.rs`) não
   aplica `world_from_local` em itens do mundo. Por isso
   `render_frontend/src/prepare/scene/world.rs::bake_t5_script_brush_poses`
   move os vértices de cada brush autorada para a pose do mapa já na carga:
   - posição nos bytes 0..12;
   - normal empacotada no 36;
   - tangente no 40.

   Cada vértice tem 44 bytes. O resultado fica em `WorldCull::bmodel_baked`
   (`BakedBrushModel`), com a pose assada, as linhas originais em espaço do
   modelo (`local_runs`, em trechos de vértices contíguos) e o `stride`.
5. **Cada frame.**
   `render_frontend/src/adapters/anim/dyn_ent_brush.rs::add_scene_brush_surfaces`
   desenha o span de superfícies de cada scene brush, como o
   `R_AddBModelSurfacesCamera` do BO1 (`append_bmodel_colour_span`).
   - Uma brush longe da pose assada (diferença > 0.01) entra em
     `WorldCull::bmodel_live` com suas linhas na pose atual
     (`BakedBrushModel::rows_at`).
   - A lista só é refeita quando alguma pose muda (gera um `Arc` novo).
   - Uma brush que saiu da cena mantém a última pose escrita.
6. **Publicação.** `render/src/extract.rs::seal_render_frame` copia o `Arc`
   (`WorldScene::brush_model_live_rows`) para
   `RenderFrameData::world_brush_rows`.
7. **GPU.**
   `render_gpu/src/drawsurf/colour_submit/geometry.rs::write_world_brush_rows`
   compara o `Arc` com o último escrito (`Arc::ptr_eq`). Se mudou:
   - devolve as linhas assadas (de `static_geometry.world_vertices`) das
     brushes que saíram da lista;
   - grava as novas com `queue.write_buffer` no `iw4_exact_colour_world_vb`,
     que agora é criado com `COPY_DST`.

   Quando o buffer é recriado (mundo novo), esse estado zera.

## Por que reescrever vértices

A alternativa seria dar uma matriz por objeto aos itens do mundo no backend,
mas isso mexe nos shaders e no agrupamento (pretess) de todos os jogos. Já
reescrever vértices fica restrito às brushes do T5 e custa pouco:

- uma folha de porta tem 24 vértices (~1 KB);
- a escrita só acontece enquanto alguma peça se mexe;
- o resto do tempo nada é enviado.

A iluminação continua a do lightmap da pose autorada.

## Colisão

O clip se mexe junto com o mover no servidor. Teste: depois de comprar a
porta, `hold +forward` atravessa o vão de x=440 até x=893. O `door_activate`
chama `NotSolid()` antes do giro.

## Como testar

Posições na Kino (ver também [testes](testes.md#posições-úteis-na-kino)):

```bash
IW4L_SOUND=off IW4L_GAMETYPE=zom ./target/play/iw4l map t5:zombie_theater --cmds 'wait world; spawn 0; god; score 5000; wait 2s; move 700 -641 322 180 5; wait 500ms; screenshot antes; move 548 -641 322 0 5; hold +activate; wait 300ms; release +activate; move 700 -641 322 180 5; wait 1s; screenshot depois; finish_run'
```

- `antes`: a porta dupla em arco, fechada, vista da sala da caixa.
- `depois`: o vão aberto e as duas folhas encostadas na parede.
- Para ver o giro em si, um log temporário em `add_scene_brush_surfaces`
  quando `bmodel_live` muda mostra a pose a cada snapshot (19 atualizações
  em 1 s). Uma screenshot trava a fila do console por ~1 s, então não dá para
  fotografar várias etapas de um giro de 1 s.
