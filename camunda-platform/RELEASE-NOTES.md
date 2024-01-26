The changelog is automatically generated using [git-chglog](https://github.com/git-chglog/git-chglog)
and it follows [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format.


<a name="camunda-platform-9.0.2"></a>
## [camunda-platform-9.0.2](https://github.com/camunda/camunda-platform-helm/compare/camunda-platform-9.0.1...camunda-platform-9.0.2) (2024-01-10)

### Fix

* update camunda/connectors-bundle docker tag to v8.4.3

## Images

Camunda images:

- docker.io/camunda/connectors-bundle:8.4.3
- docker.io/camunda/identity:8.4.0
- docker.io/camunda/operate:8.4.0
- docker.io/camunda/optimize:8.4.0
- docker.io/camunda/tasklist:8.4.0
- docker.io/camunda/zeebe:8.4.0
- registry.camunda.cloud/web-modeler-ee/modeler-restapi:8.4.1
- registry.camunda.cloud/web-modeler-ee/modeler-webapp:8.4.1
- registry.camunda.cloud/web-modeler-ee/modeler-websockets:8.4.1

Non-Camunda images:

- docker.io/bitnami/elasticsearch:8.9.2
- docker.io/bitnami/keycloak:22.0.5
- docker.io/bitnami/os-shell:11-debian-11-r93
- docker.io/bitnami/postgresql:15.5.0

### Verification

To verify integrity of the Helm chart using [Cosign](https://docs.sigstore.dev/signing/quickstart/):

```shell
cosign verify-blob .tgz \
  --bundle .cosign.bundle \
  --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
  --certificate-identity "https://github.com/camunda/camunda-platform-helm/.github/workflows/chart-release.yaml@refs/pull/1219/merge"
```
