{{/*
A template to handle constraints.
*/}}

{{- $identityEnabled := (or .Values.identity.enabled .Values.global.identity.service.url) }}
{{- $identityAuthEnabled := (or $identityEnabled .Values.global.identity.auth.enabled) }}

{{/*
Fail with a message if Multi-Tenancy is enabled and its requirements are not met which are:
- Identity chart/service.
- Identity PostgreSQL chart/service.
- Identity Auth enabled for other apps.
Multi-Tenancy requirements: https://docs.camunda.io/docs/self-managed/concepts/multi-tenancy/
*/}}
{{- if .Values.global.multitenancy.enabled }}
  {{- $identityDatabaseEnabled := (or .Values.identity.postgresql.enabled .Values.identity.externalDatabase.enabled) }}
  {{- if has false (list $identityAuthEnabled $identityDatabaseEnabled) }}
    {{- $errorMessage := printf "[camunda][error] %s %s %s %s"
        "The Multi-Tenancy feature \"global.multitenancy\" requires Identity enabled and configured with database."
        "Ensure that \"identity.enabled: true\" and \"global.identity.auth.enabled: true\""
        "and Identity database is configured built-in PostgreSQL chart via \"identity.postgresql\""
        "or configure an external database via \"identity.externalDatabase\"."
    -}}
    {{ printf "\n%s" $errorMessage | trimSuffix "\n"| fail }}
  {{- end }}
{{- end }}
