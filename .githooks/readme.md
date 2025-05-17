# Git hooks

The code in this folder will locally run at various points in your commit cycle and do useful checks before you push your code.

They need installing locally once by pointing your local git instance at them. Do this from the root of your NF_4 repository.  It will only affect this repository:

```sh
git config --local core.hooksPath .githooks/
```

After you do that, any updates to the githooks will be applied automatically so that the team has a consistent set.
