### Scanning Github repositories and docker images for secrets and API keys

# Scanning of public github repositories

- `docker run`: start a Docker container
- `--rm`: remove the container after it exits
- `-it`: interactive
- `v "$PWD:/pwd"`: mount current directory into the container at `/pwd`. This is where the results will be written
- `trufflesecurity/trufflehog:latest`: latest image for scanning
- `github`: use TruffleHog’s **GitHub integration**
- `--issue-comments --pr-comments`: include Issue comments and Pull Request comments in the scan in addition to scanning the repository content/history

```
docker run --rm -it -v "$PWD:/pwd" trufflesecurity/trufflehog:latest github --repo https://github.com/example/repo --issue-comments --pr-comments

```

P.S. To scan private repositories one will need to have a Personal Access Token (Fine-grained is preffered) and specify it like this ` -e GITHUB_TOKEN=ghp_your_personal_token_here` in the command

Extra useful parameters:

- `--only-verified` - show only **verified** findings (secret is live/valid).
- `--verify-detectors` - controls which detectors (AWS, npm token, etc) should attempt verification.
- `--format json` - choose output format
- `-o / --output <file>` - write results to a file in the mounted volume (`- v "$PWD:/pwd"` makes `/pwd` available.)

# Scanning of docker images (remote and local)

```
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  aquasec/trivy image --scanners secret trufflesecurity/secrets:latest
```

--- or with binary---

```
trufflehog docker --image trufflesecurity/secrets --results=verified
```
