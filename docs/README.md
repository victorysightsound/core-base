# /docs – Architecture Docs
High-level explanations, specifications, and design notes.

# CORE — Repository & Version Control
CORE uses **Git + GitHub** with a single `main` branch.

## SSH Authentication
A machine SSH key is used for Git pushes/pulls:
```bash
ssh-keygen -t ed25519 -C "john@victorysightsound.com"
