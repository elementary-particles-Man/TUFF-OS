# TUFF-OS: Git Push Guide (SSH)

This repo uses SSH for pushing to GitHub.

## Prerequisites
- SSH key exists at: `/mnt/Ext4-Data/ssh/id_ed25519`
- Repo root: `/mnt/Ext4-Data/Develop/TUFF-OS`

## One-time setup
Set the remote to SSH:

```
cd /mnt/Ext4-Data/Develop/TUFF-OS
git remote set-url origin git@github.com:elementary-particles-Man/TUFF-OS.git
```

## Push with SSH key
Use `GIT_SSH_COMMAND` so the correct key is used:

```
cd /mnt/Ext4-Data/Develop/TUFF-OS
GIT_SSH_COMMAND='ssh -i /mnt/Ext4-Data/ssh/id_ed25519 -o IdentitiesOnly=yes -o StrictHostKeyChecking=accept-new' git push origin main
```

If the host key is new, `StrictHostKeyChecking=accept-new` will accept it.

