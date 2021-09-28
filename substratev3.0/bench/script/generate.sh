for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done;
for i in 1 2 3 4; do for j in babe; do subkey inspect "$SECRET//$i//$j"; done; done;
for i in 1 2 3 4; do for j in grandpa; do subkey inspect --scheme ed25519 "$SECRET//$i//$j"; done; done;