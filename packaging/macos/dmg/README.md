# Janela do DMG

Arquivos usados por `package.sh` para montar a janela que o Finder mostra ao abrir o DMG:

| Arquivo | O que é |
|---|---|
| `background.tiff` | Imagem de fundo da janela (1x e 2x, HiDPI), copiada para `.background/background.tiff` na imagem. |
| `dmg-layout.DS_Store` | Ajustes do Finder (tamanho da janela, posição dos ícones, fundo), copiado para `.DS_Store` na imagem. |

Os dois são gerados por `suite-fisilab/scripts/gerar_mac.py`, em https://github.com/mrcandrade/larfic.
Não são gerados neste repositório; para mudar a janela, regenere-os lá e copie para cá.

Nomes que o layout espera (não mude sem regenerar):

- volume: `CAD` (sem versão);
- aplicativo: `CAD.app`;
- atalho para a pasta de programas: `Aplicativos`.
