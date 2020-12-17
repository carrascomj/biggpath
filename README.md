# biggpath
Annotate every [BiGG](http://bigg.ucsd.edu/) compound with the lowest pathway
in [reactome](https://reactome.org/).

## Workflow
1. Fetch all `metabolites` in BiGG.
2. For each `met` in `metabolites`:
  1. Parse `met.id_reactome`, `met.species` <- { `met` }
  2. Get low `pathway` from reactome:
  ```
  https://reactome.org/ContentService/data/pathways/low/entity/R-HSA-{met.id_reactome}?species={met.species}"
  ```
  3. Parse `pathway.dbID` <- { `pathway` }
3. Write `pathway.dbID` to file in { `met` }.

## Additional functionality
HashMap `pathway.dbID` to `pathway.displayName`.
